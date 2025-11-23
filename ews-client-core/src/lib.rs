pub mod client;

pub use client::{
    Credentials, EwsClient, EwsError, Mailbox, MessageHeaders, MessagePriority, make_header_string_for_mailbox_list,
};
