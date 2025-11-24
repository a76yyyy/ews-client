//! Message header extraction and manipulation utilities.
//!
//! This module provides a unified interface for extracting email headers from
//! different message representations (EWS messages and parsed MIME messages).

use std::iter::IntoIterator;

/// A message from which email headers can be retrieved.
///
/// This trait provides a unified interface for accessing email headers from
/// different message representations (EWS messages, parsed MIME messages, etc.).
pub trait MessageHeaders {
    /// The value of the `Message-ID` header for this message.
    fn internet_message_id(&self) -> Option<impl AsRef<str>>;

    /// Whether the message has already been read.
    fn is_read(&self) -> Option<bool>;

    /// Whether the message has any attachment.
    fn has_attachments(&self) -> Option<bool>;

    /// The time the message was sent, as a Unix timestamp converted to
    /// microseconds (compatible with Thunderbird's PRTime format).
    fn sent_timestamp_us(&self) -> Option<i64>;

    /// The author for this message. This can be the value of either the `From`
    /// or `Sender` header (in order of preference).
    fn author<'a>(&'a self) -> Option<Mailbox<'a>>;

    /// The `Reply-To` header for this message.
    fn reply_to_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>>;

    /// The `To` header for this message.
    fn to_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>>;

    /// The `Cc` header for this message.
    fn cc_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>>;

    /// The `Bcc` header for this message.
    fn bcc_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>>;

    /// The `Subject` header for this message.
    fn message_subject(&self) -> Option<impl AsRef<str>>;

    /// The message's priority/importance.
    fn priority(&self) -> Option<MessagePriority>;

    /// The messages this message refers to. This is a string which follows the
    /// format of the `References` header described in RFC822.
    fn references(&self) -> Option<impl AsRef<str>>;

    /// The size of the message in bytes.
    fn size(&self) -> Option<usize>;

    /// A short preview string for the message.
    fn preview(&self) -> Option<impl AsRef<str>>;
}

/// Implementation of MessageHeaders for EWS Message type.
impl MessageHeaders for &ews::Message {
    fn internet_message_id(&self) -> Option<impl AsRef<str>> {
        self.internet_message_id.as_ref()
    }

    fn is_read(&self) -> Option<bool> {
        self.is_read
    }

    fn has_attachments(&self) -> Option<bool> {
        self.has_attachments
    }

    fn sent_timestamp_us(&self) -> Option<i64> {
        self.date_time_sent.as_ref().and_then(|date_time| {
            // Convert Unix timestamp (seconds) to microseconds
            // This matches Thunderbird's PRTime format
            let time_in_micros = date_time.0.unix_timestamp().checked_mul(1_000 * 1_000);

            if time_in_micros.is_none() {
                let item_id = self.item_id.as_ref().map_or_else(
                    || {
                        log::error!("received message from Exchange server without an item ID");
                        "unknown"
                    },
                    |item_id| item_id.id.as_str(),
                );

                log::warn!(
                    "message with ID {} sent date {:?} too big for i64, ignoring",
                    item_id,
                    date_time
                );
            }

            time_in_micros
        })
    }

    fn author<'a>(&'a self) -> Option<Mailbox<'a>> {
        self.from
            .as_ref()
            .or(self.sender.as_ref())
            .map(|recipient| Mailbox::from(&recipient.mailbox))
    }

    fn reply_to_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>> {
        self.reply_to
            .as_ref()
            .map(|recipients| &recipients.0)
            .map(|recipients| recipients.iter().map(|recipient| Mailbox::from(&recipient.mailbox)))
    }

    fn to_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>> {
        self.to_recipients.as_ref().map(array_of_recipients_to_mailboxes)
    }

    fn cc_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>> {
        self.cc_recipients.as_ref().map(array_of_recipients_to_mailboxes)
    }

    fn bcc_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>> {
        self.bcc_recipients.as_ref().map(array_of_recipients_to_mailboxes)
    }

    fn message_subject(&self) -> Option<impl AsRef<str>> {
        self.subject.as_ref()
    }

    fn priority(&self) -> Option<MessagePriority> {
        self.importance.map(|importance| match importance {
            ews::Importance::Low => MessagePriority::Low,
            ews::Importance::Normal => MessagePriority::Normal,
            ews::Importance::High => MessagePriority::High,
        })
    }

    fn references(&self) -> Option<impl AsRef<str>> {
        self.references.as_ref()
    }

    fn size(&self) -> Option<usize> {
        self.size
    }

    fn preview(&self) -> Option<impl AsRef<str>> {
        self.preview.as_ref()
    }
}

/// Implementation of MessageHeaders for mail_parser::Message type.
impl MessageHeaders for mail_parser::Message<'_> {
    fn internet_message_id(&self) -> Option<impl AsRef<str>> {
        self.message_id()
    }

    fn is_read(&self) -> Option<bool> {
        // For parsed messages, we default to unread
        // In XPCOM version, this would read from X-Mozilla-Status header
        Some(false)
    }

    fn has_attachments(&self) -> Option<bool> {
        Some(self.attachment_count() > 0)
    }

    fn sent_timestamp_us(&self) -> Option<i64> {
        self.date()
            .and_then(|date_time| match date_time.to_timestamp().checked_mul(1_000 * 1_000) {
                Some(timestamp_us) => Some(timestamp_us),
                None => {
                    log::warn!(
                        "message with ID {} sent date {:?} too big for i64, ignoring",
                        self.message_id().unwrap_or("<none>"),
                        date_time
                    );
                    None
                }
            })
    }

    fn author<'a>(&'a self) -> Option<Mailbox<'a>> {
        self.from()
            .or(self.sender())
            .and_then(mail_parser::Address::first)
            .and_then(|addr| addr.try_into().ok())
    }

    fn reply_to_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>> {
        self.reply_to().map(|addr| addr.iter()).and_then(|addrs| {
            addrs
                .into_iter()
                .map(|addr| addr.try_into().ok())
                .collect::<Option<Vec<Mailbox<'a>>>>()
        })
    }

    fn to_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>> {
        self.to().map(address_to_mailboxes)
    }

    fn cc_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>> {
        self.cc().map(address_to_mailboxes)
    }

    fn bcc_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>> {
        self.bcc().map(address_to_mailboxes)
    }

    fn message_subject(&self) -> Option<impl AsRef<str>> {
        self.subject()
    }

    fn priority(&self) -> Option<MessagePriority> {
        self.header("X-Priority")
            .and_then(|value| value.as_text())
            .and_then(|value| value.trim().chars().next())
            .map(|first_char| {
                // X-Priority header values: 1 (highest) to 5 (lowest)
                // See https://people.dsv.su.se/~jpalme/ietf/ietf-mail-attributes.html#Heading14
                match first_char {
                    '1' => MessagePriority::Highest,
                    '2' => MessagePriority::High,
                    '3' => MessagePriority::Normal,
                    '4' => MessagePriority::Low,
                    '5' => MessagePriority::Lowest,
                    _ => MessagePriority::Normal,
                }
            })
    }

    fn references(&self) -> Option<impl AsRef<str>> {
        self.header(mail_parser::HeaderName::References)
            .and_then(|value| value.as_text())
    }

    fn size(&self) -> Option<usize> {
        Some(self.raw_message.len())
    }

    fn preview(&self) -> Option<impl AsRef<str>> {
        None::<String>
    }
}

/// Gets an iterator of mailboxes from a `mail_parser` address field, filtering
/// out any addresses which do not have an associated email address.
fn address_to_mailboxes<'a>(address: &'a mail_parser::Address) -> impl Iterator<Item = Mailbox<'a>> {
    address.iter().filter_map(|addr| addr.try_into().ok())
}

/// Gets an iterator of mailboxes from an EWS representation of a list of
/// recipients.
fn array_of_recipients_to_mailboxes<'a>(recipients: &'a ews::ArrayOfRecipients) -> impl Iterator<Item = Mailbox<'a>> {
    recipients.iter().map(|recipient| Mailbox::from(&recipient.mailbox))
}

/// Represents an email address with an optional display name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mailbox<'a> {
    /// The display name for this mailbox (e.g., "John Doe")
    pub name: Option<&'a str>,
    /// The email address (e.g., "john@example.com")
    pub email_address: Option<&'a str>,
}

impl<'a> From<&'a ews::Mailbox> for Mailbox<'a> {
    fn from(value: &'a ews::Mailbox) -> Self {
        Mailbox {
            name: value.name.as_deref(),
            email_address: value.email_address.as_deref(),
        }
    }
}

impl<'a> TryFrom<&'a mail_parser::Addr<'_>> for Mailbox<'a> {
    type Error = ();

    fn try_from(value: &'a mail_parser::Addr) -> Result<Self, Self::Error> {
        value.address.as_ref().ok_or(()).map(|address| Mailbox {
            name: value.name.as_ref().map(|name| name.as_ref()),
            email_address: Some(address.as_ref()),
        })
    }
}

impl std::fmt::Display for Mailbox<'_> {
    /// Writes the contents of the mailbox in a format suitable for use in an
    /// Internet Message Format header.
    ///
    /// The display name is encoded using RFC 2047 if it contains non-ASCII characters.
    ///
    /// Examples:
    /// - `John Doe <john@example.com>`
    /// - `john@example.com`
    /// - `=?UTF-8?B?5pil5pil?= <john@example.com>` (for non-ASCII names)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.name {
            let mut buf: Vec<u8> = Vec::new();

            // Encode the name using RFC 2047 if needed
            // This handles non-ASCII characters properly
            if let Err(e) = mail_builder::encoders::encode::rfc2047_encode(name, &mut buf) {
                log::warn!("Failed to RFC 2047 encode mailbox name: {:?}, using raw name", e);
                write!(f, "{}", name)?;
            } else {
                // RFC 2047 encoding succeeded, convert to string
                match std::str::from_utf8(&buf) {
                    Ok(encoded_name) => write!(f, "{}", encoded_name)?,
                    Err(e) => {
                        log::warn!("RFC 2047 encoded name is not valid UTF-8: {:?}, using raw name", e);
                        write!(f, "{}", name)?;
                    }
                }
            }

            if let Some(address) = self.email_address {
                write!(f, " <{}>", address)?;
            }
        } else if let Some(address) = self.email_address {
            write!(f, "{}", address)?;
        }

        Ok(())
    }
}

/// Message priority levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessagePriority {
    /// Highest priority (X-Priority: 1)
    Highest,
    /// High priority (X-Priority: 2, EWS Importance: High)
    High,
    /// Normal priority (X-Priority: 3, EWS Importance: Normal)
    Normal,
    /// Low priority (X-Priority: 4, EWS Importance: Low)
    Low,
    /// Lowest priority (X-Priority: 5)
    Lowest,
}

/// Creates a string representation of a list of mailboxes, suitable for use as
/// the value of an Internet Message Format header.
///
/// Example: `"John Doe <john@example.com>, Jane Smith <jane@example.com>"`
pub fn make_header_string_for_mailbox_list<'a>(mailboxes: impl IntoIterator<Item = Mailbox<'a>>) -> String {
    let strings: Vec<_> = mailboxes.into_iter().map(|mailbox| mailbox.to_string()).collect();

    strings.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mailbox_display_with_name_and_address() {
        let mailbox = Mailbox {
            name: Some("John Doe"),
            email_address: Some("john@example.com"),
        };
        // RFC 2047 encoding adds quotes around ASCII names
        assert_eq!(mailbox.to_string(), "\"John Doe\" <john@example.com>");
    }

    #[test]
    fn test_mailbox_display_address_only() {
        let mailbox = Mailbox {
            name: None,
            email_address: Some("john@example.com"),
        };
        assert_eq!(mailbox.to_string(), "john@example.com");
    }

    #[test]
    fn test_mailbox_display_name_only() {
        let mailbox = Mailbox {
            name: Some("John Doe"),
            email_address: None,
        };
        // RFC 2047 encoding adds quotes around ASCII names
        assert_eq!(mailbox.to_string(), "\"John Doe\"");
    }

    #[test]
    fn test_make_header_string_for_mailbox_list() {
        let mailboxes = vec![
            Mailbox {
                name: Some("John Doe"),
                email_address: Some("john@example.com"),
            },
            Mailbox {
                name: None,
                email_address: Some("jane@example.com"),
            },
        ];
        // RFC 2047 encoding adds quotes around ASCII names
        assert_eq!(
            make_header_string_for_mailbox_list(mailboxes),
            "\"John Doe\" <john@example.com>, jane@example.com"
        );
    }
}
