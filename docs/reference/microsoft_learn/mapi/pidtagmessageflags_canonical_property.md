# PidTagMessageFlags Canonical Property

**Source:** <https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/pidtagmessageflags-canonical-property>

**Applies to**: Outlook 2013 | Outlook 2016

Contains a bitmask of flags that indicate the origin and current state of a message.

## Property Details

- **Property**: PR_MESSAGE_FLAGS
- **Identifier**: 0x0E07
- **Data type**: PT_LONG
- **Area**: General messaging
- **Associated properties**: PR_HASATTACH (PidTagHasAttachments)

## Property Value Flags

One or more of the following flags can be set for this property:

| Flag | Value | Description |
| :--- | :--- | :--- |
| `MSGFLAG_ASSOCIATED` |  | The message is an associated message of a folder. The client or provider has read-only access to this flag. The MSGFLAG_READ flag is ignored for associated messages, which do not retain a read/unread state. |
| `MSGFLAG_FROMME` |  | The messaging user sending was the messaging user receiving the message. The client or provider has read/write access to this flag until the first `IMAPIProp::SaveChanges` call and read-only thereafter. This flag is meant to be set by the transport provider. |
| `MSGFLAG_HASATTACH` |  | The message has at least one attachment. This flag corresponds to the message's `PR_HASATTACH` property. The client has read-only access to this flag. |
| `MSGFLAG_NRN_PENDING` |  | A nonread report needs to be sent for the message. The client or provider has read-only access to this flag. |
| `MSGFLAG_ORIGIN_INTERNET` |  | The incoming message arrived over the Internet. It originated either outside the organization or from a source the gateway cannot consider trusted. The client should display an appropriate message to the user. Transport providers set this flag; the client has read-only access. |
| `MSGFLAG_ORIGIN_MISC_EXT` |  | The incoming message arrived over an external link other than X.400 or the Internet. It originated either outside the organization or from a source the gateway cannot consider trusted. The client should display an appropriate message to the user. Transport providers set this flag; the client has read-only access. |
| `MSGFLAG_ORIGIN_X400` |  | The incoming message arrived over an X.400 link. It originated either outside the organization or from a source the gateway cannot consider trusted. The client should display an appropriate message to the user. Transport providers set this flag; the client has read-only access. |
| `MSGFLAG_ORIGIN_EXT_SEND` |  | The message originated outside the organization. The client should display an appropriate message to the user. Transport providers set this flag; the client has read-only access. |
| `MSGFLAG_READ` |  | The message is marked as having been read. This can occur as the result of a call at any time to `IMessage::SetReadFlag` or `IMAPIFolder::SetReadFlags`. Clients can also set this flag by calling a message's `IMAPIProp::SetProps` method before the message has been saved for the first time. This flag is ignored if the `MSGFLAG_ASSOCIATED` flag is set. |
| `MSGFLAG_RESEND` |  | The message includes a request for a resend operation with a nondelivery report. The client or provider has read/write access to this flag until the first `IMAPIProp::SaveChanges` call and read-only thereafter. |
| `MSGFLAG_RN_PENDING` |  | A read report needs to be sent for the message. The client or provider has read-only access to this flag. |
| `MSGFLAG_SUBMIT` |  | The message is marked for sending as a result of a call to `IMessage::SubmitMessage`. Message store providers set this flag; the client has read-only access. |
| `MSGFLAG_UNMODIFIED` |  | The outgoing message has not been modified since the first time that it was saved; the incoming message has not been modified since it was delivered. |
| `MSGFLAG_UNSENT` |  | The message is still being composed. It is saved, but has not been sent. The client or provider has read/write access to this flag until the first `IMAPIProp::SaveChanges` call and read-only thereafter. |

## Usage

This property is used by:

- Message read/unread status tracking
- Message state management
- Transport provider operations
- Client display logic for message status

## Related Properties

- [PidTagHasAttachments Canonical Property](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/pidtaghasattachments-canonical-property)
- [PidTagMessageClass Canonical Property](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/pidtagmessageclass-canonical-property)

## See also

- [IMessage::SetReadFlag](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/imessage-setreadflag)
- [IMAPIFolder::SetReadFlags](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/imapifolder-setreadflags)
- [IMAPIProp::SetProps](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/imapiprop-setprops)
- [IMessage::SubmitMessage](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/imessage-submitmessage)
