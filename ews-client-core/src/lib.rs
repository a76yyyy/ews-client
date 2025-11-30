//! EWS (Exchange Web Services) client library.
//!
//! This crate provides a Rust client for interacting with Microsoft Exchange servers
//! using the Exchange Web Services (EWS) protocol.

pub mod client;

pub use client::{
    Credentials, EwsClient, EwsError, Mailbox, MessageHeaders, MessagePriority, make_header_string_for_mailbox_list,
};

pub use ews;
