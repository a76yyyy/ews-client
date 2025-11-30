//! SOAP response fixtures for testing
//!
//! This module provides pre-built SOAP response templates that simulate
//! real EWS server responses. These are based on the Microsoft Learn documentation
//! and the EwsServer.sys.mjs reference implementation.

#![allow(clippy::unwrap_used)]

use std::fmt::Write;

/// Common XML namespaces used in EWS responses
const NS: &str = r#"xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
                        xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types""#;

/// Common folder counts XML snippet
const FOLDER_COUNTS_XML: &str = r"<t:TotalCount>0</t:TotalCount>
              <t:ChildFolderCount>0</t:ChildFolderCount>
              <t:UnreadCount>0</t:UnreadCount>";

/// Create a SOAP envelope with the given body
macro_rules! soap {
    ($body:expr) => {
        soap!($body, "15", "20", "7452", "50")
    };
    ($body:expr, $major:expr, $minor:expr) => {
        soap!($body, $major, $minor, "7452", "50")
    };
    ($body:expr, $major:expr, $minor:expr, $major_build:expr, $minor_build:expr) => {
        format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
  <s:Header>
    <h:ServerVersionInfo MajorVersion="{}" MinorVersion="{}" MajorBuildNumber="{}" MinorBuildNumber="{}"
      xmlns:h="http://schemas.microsoft.com/exchange/services/2006/types"
      xmlns:xsd="http://www.w3.org/2001/XMLSchema"
      xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>
  </s:Header>
  <s:Body>
    {}
  </s:Body>
</s:Envelope>"#,
            $major, $minor, $major_build, $minor_build, $body
        )
    };
}

/// Create a generic ID XML element
macro_rules! id_xml {
    ($tag:expr, $id:expr, $ck:expr) => {
        format!(r#"<t:{0} Id="{1}" ChangeKey="{2}" />"#, $tag, $id, $ck)
    };
}

/// Create a standard `ResponseMessage` XML
macro_rules! response_message_xml {
    ($op:expr, $class:expr, $code:expr, $content:expr) => {
        format!(
            r#"<m:{0}ResponseMessage ResponseClass="{1}">
          <m:ResponseCode>{2}</m:ResponseCode>
          {3}
        </m:{0}ResponseMessage>"#,
            $op, $class, $code, $content
        )
    };
}

/// Create a standard error `ResponseMessage` XML snippet (without `op_response` wrapper)
macro_rules! response_error_xml {
    ($op:expr, $code:expr, $msg:expr) => {
        format!(
            r#"<m:{0}ResponseMessage ResponseClass="Error">
          <m:MessageText>{1}</m:MessageText>
          <m:ResponseCode>{2}</m:ResponseCode>
          <m:DescriptiveLinkKey>0</m:DescriptiveLinkKey>
        </m:{0}ResponseMessage>"#,
            $op, $msg, $code
        )
    };
}

/// Create a full Operation Response with messages
macro_rules! op_response {
    ($op:expr, $messages:expr) => {
        soap!(format!(
            r"<m:{0}Response {1}>
      <m:ResponseMessages>
        {2}
      </m:ResponseMessages>
    </m:{0}Response>",
            $op, NS, $messages
        ))
    };
    ($op:expr, $messages:expr, $major:expr, $minor:expr) => {
        soap!(
            format!(
                r"<m:{0}Response {1}>
      <m:ResponseMessages>
        {2}
      </m:ResponseMessages>
    </m:{0}Response>",
                $op, NS, $messages
            ),
            $major,
            $minor
        )
    };
    ($op:expr, $messages:expr, $major:expr, $minor:expr, $major_build:expr, $minor_build:expr) => {
        soap!(
            format!(
                r"<m:{0}Response {1}>
      <m:ResponseMessages>
        {2}
      </m:ResponseMessages>
    </m:{0}Response>",
                $op, NS, $messages
            ),
            $major,
            $minor,
            $major_build,
            $minor_build
        )
    };
}

/// Create a container XML snippet
macro_rules! container_xml {
    ($container:expr, $element:expr, $content:expr) => {
        format!(
            r"<m:{0}>
            <t:{1}>
              {2}
            </t:{1}>
          </m:{0}>",
            $container, $element, $content
        )
    };
}

/// Create a standard item success XML snippet
macro_rules! item_success_xml {
    ($item_id:expr) => {
        item_success_xml!("Message", $item_id, "CQAAAA==")
    };
    ($item_id:expr, $ck:expr) => {
        item_success_xml!("Message", $item_id, $ck)
    };
    ($type:expr, $item_id:expr, $ck:expr) => {
        container_xml!("Items", $type, id_xml!("ItemId", $item_id, $ck))
    };
}

/// Create a standard folder success XML snippet
macro_rules! folder_success_xml {
    ($folder_id:expr) => {
        folder_success_xml!($folder_id, "AQAAAA==")
    };
    ($folder_id:expr, $ck:expr) => {
        container_xml!("Folders", "Folder", id_xml!("FolderId", $folder_id, $ck))
    };
}

/// Create a standard update item success XML snippet
macro_rules! update_item_success_xml {
    ($item_id:expr, $ck:expr) => {
        update_item_success_xml!("Message", $item_id, $ck)
    };
    ($type:expr, $item_id:expr, $ck:expr) => {
        format!(
            r"{}
          <m:ConflictResults>
            <t:Count>0</t:Count>
          </m:ConflictResults>",
            container_xml!("Items", $type, id_xml!("ItemId", $item_id, $ck))
        )
    };
}

/// Create a success response with folder
macro_rules! folder_success {
    ($op:expr, $folder_id:expr) => {
        op_response!(
            $op,
            response_message_xml!($op, "Success", "NoError", folder_success_xml!($folder_id))
        )
    };
}

/// Create a success response with item
macro_rules! item_success {
    ($op:expr, $item_id:expr) => {
        item_success!($op, "Message", $item_id)
    };
    ($op:expr, $type:expr, $item_id:expr) => {
        op_response!(
            $op,
            response_message_xml!(
                $op,
                "Success",
                "NoError",
                item_success_xml!($type, $item_id, "CQAAAA==")
            )
        )
    };
}

/// Create a simple success response (no items/folders)
macro_rules! simple_success {
    ($op:expr) => {
        op_response!($op, response_message_xml!($op, "Success", "NoError", ""))
    };
}

/// Create a standard error response
macro_rules! error_response {
    ($op:expr, $code:expr, $msg:expr) => {
        op_response!(
            $op,
            format!(
                r#"<m:{0}ResponseMessage ResponseClass="Error">
          <m:MessageText>{1}</m:MessageText>
          <m:ResponseCode>{2}</m:ResponseCode>
          <m:DescriptiveLinkKey>0</m:DescriptiveLinkKey>
        </m:{0}ResponseMessage>"#,
                $op, $msg, $code
            )
        )
    };
    ($op:expr, $code:expr, $msg:expr, $major:expr, $minor:expr) => {
        op_response!(
            $op,
            format!(
                r#"<m:{0}ResponseMessage ResponseClass="Error">
          <m:MessageText>{1}</m:MessageText>
          <m:ResponseCode>{2}</m:ResponseCode>
          <m:DescriptiveLinkKey>0</m:DescriptiveLinkKey>
        </m:{0}ResponseMessage>"#,
                $op, $msg, $code
            ),
            $major,
            $minor
        )
    };
    ($op:expr, $code:expr, $msg:expr, $major:expr, $minor:expr, $major_build:expr, $minor_build:expr) => {
        op_response!(
            $op,
            format!(
                r#"<m:{0}ResponseMessage ResponseClass="Error">
          <m:MessageText>{1}</m:MessageText>
          <m:ResponseCode>{2}</m:ResponseCode>
          <m:DescriptiveLinkKey>0</m:DescriptiveLinkKey>
        </m:{0}ResponseMessage>"#,
                $op, $msg, $code
            ),
            $major,
            $minor,
            $major_build,
            $minor_build
        )
    };
}

/// Create a standard success response wrapper
macro_rules! success_response {
    ($op:expr, $content:expr) => {
        op_response!($op, response_message_xml!($op, "Success", "NoError", $content))
    };
}

/// Create a standard folder XML snippet
macro_rules! folder_xml {
    ($id:expr, $name:expr) => {
        format!(
            r"<t:Folder>
              {}
              <t:DisplayName>{}</t:DisplayName>
              {}
            </t:Folder>",
            id_xml!("FolderId", $id, "AQAAAA=="),
            $name,
            FOLDER_COUNTS_XML
        )
    };
}

/// Create a full folder XML snippet with `ParentFolderId` and `FolderClass`
macro_rules! folder_full_xml {
    ($id:expr, $parent_id:expr, $name:expr, $class:expr) => {
        format!(
            r"<t:Folder>
              {}
              {}
              <t:FolderClass>{}</t:FolderClass>
              <t:DisplayName>{}</t:DisplayName>
              {}
            </t:Folder>",
            id_xml!("FolderId", $id, "AQAAAA=="),
            id_xml!("ParentFolderId", $parent_id, "AQAAAA=="),
            $class,
            $name,
            FOLDER_COUNTS_XML
        )
    };
}

/// Create a SOAP Fault response
macro_rules! fault_response {
    ($code:expr, $string:expr, $detail_code:expr, $detail_msg:expr) => {
        fault_response!($code, $string, $detail_code, $detail_msg, "")
    };
    ($code:expr, $string:expr, $detail_code:expr, $detail_msg:expr, $extra:expr) => {
        soap!(format!(
            r#"<s:Fault xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
      <faultcode>{}</faultcode>
      <faultstring>{}</faultstring>
      <detail>
        <t:ResponseCode xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">{}</t:ResponseCode>
        <t:Message xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">{}</t:Message>
        {}
      </detail>
    </s:Fault>"#,
            $code, $string, $detail_code, $detail_msg, $extra
        ))
    };
}

/// Create a success response for Find operations (with `RootFolder`)
macro_rules! find_success {
    ($op:expr, $total:expr, $includes_last:expr, $content:expr) => {
        op_response!(
            $op,
            response_message_xml!(
                $op,
                "Success",
                "NoError",
                format!(
                    r#"<m:RootFolder TotalItemsInView="{}" IncludesLastItemInRange="{}">
            {}
          </m:RootFolder>"#,
                    $total, $includes_last, $content
                )
            )
        )
    };
}

/// Create a success response for Update operations (with `ConflictResults`)
macro_rules! update_success {
    ($op:expr, $item_id:expr) => {
        update_success!($op, $item_id, "CQAAAA==")
    };
    ($op:expr, $item_id:expr, $ck:expr) => {
        op_response!(
            $op,
            response_message_xml!($op, "Success", "NoError", update_item_success_xml!($item_id, $ck))
        )
    };
}

// ============================================================================
// Folder Operations Fixtures
// ============================================================================

/// Response for successful `CreateFolder` operation
pub fn create_folder_response(folder_id: &str) -> String {
    folder_success!("CreateFolder", folder_id)
}

/// Response for successful `DeleteFolder` operation
pub fn delete_folder_response() -> String {
    simple_success!("DeleteFolder")
}

/// Response for successful `GetFolder` operation
pub fn get_folder_response(folder_id: &str, display_name: &str) -> String {
    success_response!(
        "GetFolder",
        format!(
            r"<m:Folders>
            {}
          </m:Folders>",
            folder_xml!(folder_id, display_name)
        )
    )
}

/// Response for successful `GetFolder` operation with full details
pub fn get_folder_full_response(folder_id: &str, parent_id: &str, display_name: &str, folder_class: &str) -> String {
    success_response!(
        "GetFolder",
        format!(
            r"<m:Folders>
            {}
          </m:Folders>",
            folder_full_xml!(folder_id, parent_id, display_name, folder_class)
        )
    )
}

/// Response for `GetFolder` with distinguished folder ID (used in `check_connectivity`)
pub fn get_folder_distinguished_response(distinguished_id: &str, folder_id: &str) -> String {
    success_response!(
        "GetFolder",
        format!(
            r"<m:Folders>
            {}
          </m:Folders>",
            folder_xml!(folder_id, distinguished_id)
        )
    )
}

/// Response for batch `GetFolder` operation with multiple folders
pub fn batch_get_folder_response(folder_ids: &[&str], display_names: &[&str]) -> String {
    let messages = folder_ids
        .iter()
        .zip(display_names.iter())
        .map(|(id, name)| {
            response_message_xml!(
                "GetFolder",
                "Success",
                "NoError",
                format!(
                    r"<m:Folders>
            {}
          </m:Folders>",
                    folder_full_xml!(id, "root-folder-id", name, "IPF.Note")
                )
            )
        })
        .collect::<String>();

    op_response!("GetFolder", messages)
}

/// Response for successful `SyncFolderHierarchy` operation (initial sync)
pub fn sync_folder_hierarchy_response(sync_state: &str, folder_id: &str) -> String {
    success_response!(
        "SyncFolderHierarchy",
        format!(
            r"<m:SyncState>{}</m:SyncState>
          <m:IncludesLastFolderInRange>true</m:IncludesLastFolderInRange>
          <m:Changes>
            <t:Create>
              {}
            </t:Create>
          </m:Changes>",
            sync_state,
            folder_full_xml!(folder_id, "root-folder-id", "TestFolder", "IPF.Note")
        )
    )
}

/// Response for successful `SyncFolderHierarchy` with multiple change types
pub fn sync_folder_hierarchy_with_changes_response(
    sync_state: &str,
    created_id: &str,
    updated_id: &str,
    deleted_id: &str,
) -> String {
    success_response!(
        "SyncFolderHierarchy",
        format!(
            r"<m:SyncState>{}</m:SyncState>
          <m:IncludesLastFolderInRange>true</m:IncludesLastFolderInRange>
          <m:Changes>
            <t:Create>
              <t:Folder>
                {}
                {}
                <t:DisplayName>NewFolder</t:DisplayName>
                <t:FolderClass>IPF.Note</t:FolderClass>
              </t:Folder>
            </t:Create>
            <t:Update>
              <t:Folder>
                {}
                {}
                <t:DisplayName>UpdatedFolder</t:DisplayName>
                <t:FolderClass>IPF.Note</t:FolderClass>
              </t:Folder>
            </t:Update>
            <t:Delete>
              {}
            </t:Delete>
          </m:Changes>",
            sync_state,
            id_xml!("FolderId", created_id, "AQAAAA=="),
            id_xml!("ParentFolderId", "root-folder-id", "AQAAAA=="),
            id_xml!("FolderId", updated_id, "AQAAAB=="),
            id_xml!("ParentFolderId", "root-folder-id", "AQAAAA=="),
            id_xml!("FolderId", deleted_id, "AQAAAC==")
        )
    )
}

/// Response for `SyncFolderHierarchy` with pagination (IncludesLastFolderInRange=false)
pub fn sync_folder_hierarchy_paginated_response(_sync_state: &str, next_sync_state: &str, folder_id: &str) -> String {
    success_response!(
        "SyncFolderHierarchy",
        format!(
            r"<m:SyncState>{}</m:SyncState>
          <m:IncludesLastFolderInRange>false</m:IncludesLastFolderInRange>
          <m:Changes>
            <t:Create>
              <t:Folder>
                {}
                {}
                <t:DisplayName>PaginatedFolder</t:DisplayName>
                <t:FolderClass>IPF.Note</t:FolderClass>
              </t:Folder>
            </t:Create>
          </m:Changes>",
            next_sync_state,
            id_xml!("FolderId", folder_id, "AQAAAA=="),
            id_xml!("ParentFolderId", "root-folder-id", "AQAAAA==")
        )
    )
}

// ============================================================================
// Item Operations Fixtures
// ============================================================================

/// Response for successful `CreateItem` operation
pub fn create_item_response(item_id: &str) -> String {
    item_success!("CreateItem", item_id)
}

/// Response for successful `CreateItem` operation for `CalendarItem`
pub fn create_calendar_item_response(item_id: &str) -> String {
    item_success!("CreateItem", "CalendarItem", item_id)
}

/// Response for successful `CreateItem` operation for Contact
pub fn create_contact_response(item_id: &str) -> String {
    item_success!("CreateItem", "Contact", item_id)
}

/// Response for successful `CreateItem` operation for Task
pub fn create_task_response(item_id: &str) -> String {
    item_success!("CreateItem", "Task", item_id)
}

/// Response for successful `DeleteItem` operation
pub fn delete_item_response() -> String {
    simple_success!("DeleteItem")
}

/// Response for successful `GetItem` operation
pub fn get_item_response(item_id: &str, subject: &str) -> String {
    success_response!(
        "GetItem",
        format!(
            r"<m:Items>
            <t:Message>
              {}
              <t:Subject>{}</t:Subject>
              <t:IsRead>false</t:IsRead>
            </t:Message>
          </m:Items>",
            id_xml!("ItemId", item_id, "CQAAAA=="),
            subject
        )
    )
}

/// Response for `GetItem` with MIME content (used in `get_message`)
pub fn get_item_with_mime_response(item_id: &str, subject: &str, mime_content: &str) -> String {
    success_response!(
        "GetItem",
        format!(
            r#"<m:Items>
            <t:Message>
              <t:MimeContent CharacterSet="UTF-8">{}</t:MimeContent>
              {}
              <t:Subject>{}</t:Subject>
              <t:Size>1024</t:Size>
              <t:DateTimeSent>2024-01-15T10:30:00Z</t:DateTimeSent>
              <t:HasAttachments>false</t:HasAttachments>
              <t:From>
                <t:Mailbox>
                  <t:Name>John Doe</t:Name>
                  <t:EmailAddress>john@example.com</t:EmailAddress>
                </t:Mailbox>
              </t:From>
              <t:IsRead>false</t:IsRead>
            </t:Message>
          </m:Items>"#,
            mime_content,
            id_xml!("ItemId", item_id, "CQAAAA=="),
            subject
        )
    )
}

/// Response for `GetItem` with full message details
#[allow(clippy::too_many_arguments)]
pub fn get_item_full_response(
    item_id: &str,
    subject: &str,
    from_name: &str,
    from_email: &str,
    is_read: bool,
    has_attachments: bool,
    size: usize,
    importance: &str,
    references: &str,
) -> String {
    success_response!(
        "GetItem",
        format!(
            r"<m:Items>
            <t:Message>
              {}
              <t:Subject>{}</t:Subject>
              <t:Size>{}</t:Size>
              <t:Importance>{}</t:Importance>
              <t:DateTimeSent>2024-01-15T10:30:00Z</t:DateTimeSent>
              <t:HasAttachments>{}</t:HasAttachments>
              <t:From>
                <t:Mailbox>
                  <t:Name>{}</t:Name>
                  <t:EmailAddress>{}</t:EmailAddress>
                </t:Mailbox>
              </t:From>
              <t:InternetMessageId>message-id-123@example.com</t:InternetMessageId>
              <t:IsRead>{}</t:IsRead>
              <t:References>{}</t:References>
            </t:Message>
          </m:Items>",
            id_xml!("ItemId", item_id, "CQAAAA=="),
            subject,
            size,
            importance,
            has_attachments,
            from_name,
            from_email,
            is_read,
            references
        )
    )
}

/// Response for `GetItem` with recipients (To, Cc, Bcc, `ReplyTo`)
#[allow(clippy::too_many_arguments)]
pub fn get_item_with_recipients_response(
    item_id: &str,
    subject: &str,
    to_name: &str,
    to_email: &str,
    cc_name: &str,
    cc_email: &str,
    bcc_name: &str,
    bcc_email: &str,
    reply_to_name: &str,
    reply_to_email: &str,
) -> String {
    success_response!(
        "GetItem",
        format!(
            r"<m:Items>
            <t:Message>
              {}
              <t:Subject>{}</t:Subject>
              <t:ToRecipients>
                <t:Mailbox>
                  <t:Name>{}</t:Name>
                  <t:EmailAddress>{}</t:EmailAddress>
                </t:Mailbox>
              </t:ToRecipients>
              <t:CcRecipients>
                <t:Mailbox>
                  <t:Name>{}</t:Name>
                  <t:EmailAddress>{}</t:EmailAddress>
                </t:Mailbox>
              </t:CcRecipients>
              <t:BccRecipients>
                <t:Mailbox>
                  <t:Name>{}</t:Name>
                  <t:EmailAddress>{}</t:EmailAddress>
                </t:Mailbox>
              </t:BccRecipients>
              <t:ReplyTo>
                <t:Mailbox>
                  <t:Name>{}</t:Name>
                  <t:EmailAddress>{}</t:EmailAddress>
                </t:Mailbox>
              </t:ReplyTo>
            </t:Message>
          </m:Items>",
            id_xml!("ItemId", item_id, "CQAAAA=="),
            subject,
            to_name,
            to_email,
            cc_name,
            cc_email,
            bcc_name,
            bcc_email,
            reply_to_name,
            reply_to_email
        )
    )
}

/// Response for `GetItem` with extended properties
pub fn get_item_with_extended_properties_response(item_id: &str, prop_tag: &str, prop_value: &str) -> String {
    success_response!(
        "GetItem",
        format!(
            r#"<m:Items>
            <t:Message>
              {}
              <t:ExtendedProperty>
                <t:ExtendedFieldURI PropertyTag="{}" PropertyType="String" />
                <t:Value>{}</t:Value>
              </t:ExtendedProperty>
            </t:Message>
          </m:Items>"#,
            id_xml!("ItemId", item_id, "CQAAAA=="),
            prop_tag,
            prop_value
        )
    )
}

/// Response for `GetItem` with Body
pub fn get_item_with_body_response(item_id: &str, body_type: &str, body_content: &str) -> String {
    success_response!(
        "GetItem",
        format!(
            r#"<m:Items>
            <t:Message>
              {}
              <t:Body BodyType="{}">{}</t:Body>
            </t:Message>
          </m:Items>"#,
            id_xml!("ItemId", item_id, "CQAAAA=="),
            body_type,
            body_content
        )
    )
}

/// Response for successful `SyncFolderItems` operation
pub fn sync_folder_items_response(sync_state: &str, item_id: &str) -> String {
    success_response!(
        "SyncFolderItems",
        format!(
            r"<m:SyncState>{}</m:SyncState>
          <m:IncludesLastItemInRange>true</m:IncludesLastItemInRange>
          <m:Changes>
            <t:Create>
              <t:Message>
                {}
              </t:Message>
            </t:Create>
          </m:Changes>",
            sync_state,
            id_xml!("ItemId", item_id, "CQAAAA==")
        )
    )
}

/// Response for successful `SyncFolderItems` with multiple change types
pub fn sync_folder_items_with_changes_response(
    sync_state: &str,
    created_id: &str,
    updated_id: &str,
    deleted_id: &str,
) -> String {
    success_response!(
        "SyncFolderItems",
        format!(
            r"<m:SyncState>{}</m:SyncState>
          <m:IncludesLastItemInRange>true</m:IncludesLastItemInRange>
          <m:Changes>
            <t:Create>
              <t:Message>
                {}
              </t:Message>
            </t:Create>
            <t:Update>
              <t:Message>
                {}
              </t:Message>
            </t:Update>
            <t:Delete>
              {}
            </t:Delete>
            <t:ReadFlagChange>
              {}
              <t:IsRead>true</t:IsRead>
            </t:ReadFlagChange>
          </m:Changes>",
            sync_state,
            id_xml!("ItemId", created_id, "CQAAAA=="),
            id_xml!("ItemId", updated_id, "CQAAAB=="),
            id_xml!("ItemId", deleted_id, "CQAAAC=="),
            id_xml!("ItemId", "read-flag-item-id", "CQAAAD==")
        )
    )
}

/// Response for `SyncFolderItems` with pagination (IncludesLastItemInRange=false)
pub fn sync_folder_items_paginated_response(_sync_state: &str, next_sync_state: &str, item_id: &str) -> String {
    success_response!(
        "SyncFolderItems",
        format!(
            r"<m:SyncState>{}</m:SyncState>
          <m:IncludesLastItemInRange>false</m:IncludesLastItemInRange>
          <m:Changes>
            <t:Create>
              <t:Message>
                {}
              </t:Message>
            </t:Create>
          </m:Changes>",
            next_sync_state,
            id_xml!("ItemId", item_id, "CQAAAA==")
        )
    )
}

// ============================================================================
// Copy/Move Operations Fixtures
// ============================================================================

/// Response for successful `CopyItem` operation
pub fn copy_item_response(item_id: &str) -> String {
    item_success!("CopyItem", item_id)
}

/// Response for successful `MoveItem` operation
pub fn move_item_response(item_id: &str) -> String {
    let new_id = format!("moved-{item_id}");
    item_success!("MoveItem", new_id)
}

/// Response for successful `CopyFolder` operation
pub fn copy_folder_response(folder_id: &str) -> String {
    folder_success!("CopyFolder", folder_id)
}

/// Response for successful `MoveFolder` operation
pub fn move_folder_response(folder_id: &str) -> String {
    let new_id = format!("moved-{folder_id}");
    folder_success!("MoveFolder", new_id)
}

// ============================================================================
// Special Operations Fixtures
// ============================================================================

/// Response for successful `MarkAsJunk` operation
pub fn mark_as_junk_response(moved_item_id: Option<&str>) -> String {
    let content = match moved_item_id {
        Some(id) => format!(r#"<m:MovedItemId Id="{id}" ChangeKey="CQAAAA==" />"#),
        None => String::new(),
    };

    op_response!(
        "MarkAsJunk",
        response_message_xml!("MarkAsJunk", "Success", "NoError", content)
    )
}

/// Response for successful `MarkAllItemsAsRead` operation
pub fn mark_all_items_as_read_response() -> String {
    simple_success!("MarkAllItemsAsRead")
}

/// Response for successful `UpdateItem` operation
pub fn update_item_response(item_id: &str) -> String {
    update_success!("UpdateItem", item_id)
}

/// Response for successful `UpdateItem` with `SetItemField` (Set action)
pub fn update_item_set_field_response(item_id: &str) -> String {
    update_success!("UpdateItem", item_id, "CQAAAB==")
}

/// Response for successful `UpdateItem` with `AppendToItemField` (Append action)
pub fn update_item_append_field_response(item_id: &str) -> String {
    update_success!("UpdateItem", item_id, "CQAAAC==")
}

/// Response for successful `UpdateItem` with `DeleteItemField` (Delete action)
pub fn update_item_delete_field_response(item_id: &str) -> String {
    update_success!("UpdateItem", item_id, "CQAAAD==")
}

/// Response for successful `UpdateItemInRecoverableItems` operation
pub fn update_item_in_recoverable_items_response(item_id: &str) -> String {
    op_response!(
        "UpdateItemInRecoverableItems",
        response_message_xml!(
            "UpdateItemInRecoverableItems",
            "Success",
            "NoError",
            format!(
                r"{}
          <m:ConflictResults>
            <t:Count>0</t:Count>
          </m:ConflictResults>",
                container_xml!("Items", "Message", id_xml!("ItemId", item_id, "CQAAAA=="))
            )
        )
    )
}

/// Response for successful `UpdateFolder` operation
pub fn update_folder_response(folder_id: &str) -> String {
    folder_success!("UpdateFolder", folder_id)
}

/// Response for successful `SendItem` operation
pub fn send_item_response() -> String {
    simple_success!("SendItem")
}

/// Response for successful `CreateItem` with MessageDisposition=SendOnly (used in `send_message`)
pub fn create_item_send_response() -> String {
    simple_success!("CreateItem")
}

// ============================================================================
// Search Operations Fixtures
// ============================================================================

/// Response for successful `FindFolder` operation
pub fn find_folder_response(folder_id: &str, display_name: &str) -> String {
    find_success!(
        "FindFolder",
        1,
        true,
        format!(
            r"<t:Folders>
              {}
            </t:Folders>",
            folder_xml!(folder_id, display_name)
        )
    )
}

/// Response for successful `FindFolder` with multiple results and pagination
pub fn find_folder_paginated_response(
    folder_ids: &[&str],
    display_names: &[&str],
    total_items: usize,
    includes_last: bool,
) -> String {
    let folders = folder_ids
        .iter()
        .zip(display_names.iter())
        .map(|(id, name)| folder_xml!(id, name))
        .collect::<String>();

    find_success!(
        "FindFolder",
        total_items,
        includes_last,
        format!(
            r"<t:Folders>
              {}
            </t:Folders>",
            folders
        )
    )
}

/// Response for successful `FindItem` operation
pub fn find_item_response(item_id: &str) -> String {
    find_success!(
        "FindItem",
        1,
        true,
        format!(
            r"<t:Items>
              <t:Message>
                {}
              </t:Message>
            </t:Items>",
            id_xml!("ItemId", item_id, "CQAAAA==")
        )
    )
}

/// Response for successful `FindItem` with multiple results and pagination
pub fn find_item_paginated_response(item_ids: &[&str], total_items: usize, includes_last: bool) -> String {
    let items = item_ids.iter().fold(String::new(), |mut output, id| {
        let _ = write!(
            output,
            r"<t:Message>
                {}
              </t:Message>",
            id_xml!("ItemId", id, "CQAAAA==")
        );
        output
    });

    find_success!(
        "FindItem",
        total_items,
        includes_last,
        format!(
            r"<t:Items>
              {}
            </t:Items>",
            items
        )
    )
}

// ============================================================================
// Error Response Fixtures
// ============================================================================

/// Response for invalid folder ID error
pub fn error_invalid_folder_id() -> String {
    error_response!("GetFolder", "ErrorInvalidIdMalformed", "Id is malformed.")
}

/// Response for folder not found error
pub fn error_folder_not_found() -> String {
    error_response!(
        "GetFolder",
        "ErrorFolderNotFound",
        "The specified object was not found in the store."
    )
}

/// Response for item not found error
pub fn error_item_not_found() -> String {
    error_response!(
        "GetItem",
        "ErrorItemNotFound",
        "The specified object was not found in the store."
    )
}

/// Response for access denied error
pub fn error_access_denied() -> String {
    error_response!("GetFolder", "ErrorAccessDenied", "Access is denied.")
}

/// Response for server busy error
pub fn error_server_busy() -> String {
    fault_response!(
        "ErrorServerBusy",
        "The server is busy.",
        "ErrorServerBusy",
        "The server is busy.",
        r#"<m:MessageXml xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages" xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
          <t:Value Name="BackOffMilliseconds">5000</t:Value>
        </m:MessageXml>"#
    )
}

/// Response for invalid request error
pub fn error_invalid_request() -> String {
    error_response!("GetFolder", "ErrorInvalidRequest", "The request is invalid.")
}

/// Response for authentication failed error
pub fn error_authentication_failed() -> String {
    fault_response!(
        "ErrorAuthenticationFailed",
        "Authentication failed.",
        "ErrorAuthenticationFailed",
        "Authentication failed."
    )
}

/// Response for quota exceeded error
pub fn error_quota_exceeded() -> String {
    error_response!(
        "GetFolder",
        "ErrorQuotaExceeded",
        "The mailbox quota has been exceeded."
    )
}

/// Response for insufficient resources error
pub fn error_insufficient_resources() -> String {
    error_response!(
        "GetFolder",
        "ErrorInsufficientResources",
        "Insufficient resources to complete the operation."
    )
}

/// Response for mailbox store unavailable error
pub fn error_mailbox_store_unavailable() -> String {
    error_response!(
        "GetFolder",
        "ErrorMailboxStoreUnavailable",
        "The mailbox store is unavailable."
    )
}

/// Response for folder not empty error
pub fn error_folder_not_empty() -> String {
    error_response!("DeleteFolder", "ErrorFolderNotEmpty", "The folder is not empty.")
}

/// Response for invalid change key error
pub fn error_invalid_change_key() -> String {
    error_response!("UpdateItem", "ErrorInvalidChangeKey", "The change key is invalid.")
}

/// Response for sync state invalid error
pub fn error_invalid_sync_state() -> String {
    error_response!(
        "SyncFolderHierarchy",
        "ErrorInvalidSyncStateData",
        "The sync state is invalid."
    )
}

/// Response for `MarkAsJunk` not supported error (Exchange < 2013)
pub fn error_mark_as_junk_not_supported() -> String {
    error_response!(
        "MarkAsJunk",
        "ErrorInvalidRequest",
        "The MarkAsJunk operation is not supported on this server version.",
        "14",
        "03",
        "0123",
        "000"
    )
}

/// Response for batch operation with mixed success and error
pub fn batch_get_folder_mixed_response(success_folder_id: &str, _error_folder_id: &str) -> String {
    let success_msg = response_message_xml!(
        "GetFolder",
        "Success",
        "NoError",
        format!(
            r"<m:Folders>
            <t:Folder>
              {}
              <t:DisplayName>SuccessFolder</t:DisplayName>
              <t:TotalCount>5</t:TotalCount>
              <t:ChildFolderCount>2</t:ChildFolderCount>
              <t:UnreadCount>3</t:UnreadCount>
            </t:Folder>
          </m:Folders>",
            id_xml!("FolderId", success_folder_id, "AQAAAA==")
        )
    );

    let error_msg = response_error_xml!(
        "GetFolder",
        "ErrorFolderNotFound",
        "The specified object was not found in the store."
    );

    op_response!("GetFolder", format!("{}{}", success_msg, error_msg))
}

/// Response for batch get item operation with multiple items
pub fn batch_get_item_response(item_ids: &[&str], subjects: &[&str]) -> String {
    let messages = item_ids
        .iter()
        .zip(subjects.iter())
        .map(|(id, subject)| {
            response_message_xml!(
                "GetItem",
                "Success",
                "NoError",
                format!(
                    r"<m:Items>
            <t:Message>
              {}
              <t:Subject>{}</t:Subject>
              <t:IsRead>false</t:IsRead>
            </t:Message>
          </m:Items>",
                    id_xml!("ItemId", id, "CQAAAA=="),
                    subject
                )
            )
        })
        .collect::<String>();

    op_response!("GetItem", messages)
}

/// Response for batch delete item operation
pub fn batch_delete_item_response(count: usize) -> String {
    let messages = (0..count)
        .map(|_| response_message_xml!("DeleteItem", "Success", "NoError", ""))
        .collect::<String>();

    op_response!("DeleteItem", messages)
}

/// Response for batch delete item operation with mixed success and error
pub fn batch_delete_item_mixed_response(success_count: usize, error_count: usize) -> String {
    let mut messages = (0..success_count)
        .map(|_| response_message_xml!("DeleteItem", "Success", "NoError", ""))
        .collect::<String>();

    let error_messages = (0..error_count)
        .map(|_| {
            response_error_xml!(
                "DeleteItem",
                "ErrorItemNotFound",
                "The specified object was not found in the store."
            )
        })
        .collect::<String>();

    messages.push_str(&error_messages);

    op_response!("DeleteItem", messages)
}

/// Response for batch update item operation
pub fn batch_update_item_response(item_ids: &[&str]) -> String {
    let messages = item_ids
        .iter()
        .enumerate()
        .map(|(idx, id)| {
            let change_key = format!("CQAA{idx:02X}==");
            response_message_xml!(
                "UpdateItem",
                "Success",
                "NoError",
                update_item_success_xml!(id, change_key)
            )
        })
        .collect::<String>();

    op_response!("UpdateItem", messages)
}

/// Response for batch update item operation with mixed success and error
pub fn batch_update_item_mixed_response(success_ids: &[&str], error_count: usize) -> String {
    let mut messages = success_ids
        .iter()
        .enumerate()
        .map(|(idx, id)| {
            let change_key = format!("CQAA{idx:02X}==");
            response_message_xml!(
                "UpdateItem",
                "Success",
                "NoError",
                update_item_success_xml!(id, change_key)
            )
        })
        .collect::<String>();

    let error_messages = (0..error_count)
        .map(|_| {
            response_error_xml!(
                "UpdateItem",
                "ErrorItemNotFound",
                "The specified object was not found in the store."
            )
        })
        .collect::<String>();

    messages.push_str(&error_messages);

    op_response!("UpdateItem", messages)
}

/// Response for batch mark as junk operation
pub fn batch_mark_as_junk_response(item_ids: &[&str]) -> String {
    let messages = item_ids
        .iter()
        .enumerate()
        .map(|(idx, id)| {
            let change_key = format!("CQAA{idx:02X}==");
            response_message_xml!(
                "MarkAsJunk",
                "Success",
                "NoError",
                format!(r#"<m:MovedItemId Id="{}" ChangeKey="{}" />"#, id, change_key)
            )
        })
        .collect::<String>();

    op_response!("MarkAsJunk", messages)
}

/// Response for batch copy item operation
pub fn batch_copy_item_response(item_ids: &[&str]) -> String {
    let messages = item_ids
        .iter()
        .enumerate()
        .map(|(idx, id)| {
            let new_id = format!("copied-{id}");
            let change_key = format!("CQAA{idx:02X}==");
            response_message_xml!("CopyItem", "Success", "NoError", item_success_xml!(new_id, change_key))
        })
        .collect::<String>();

    op_response!("CopyItem", messages)
}

/// Response for batch move item operation
pub fn batch_move_item_response(item_ids: &[&str]) -> String {
    let messages = item_ids
        .iter()
        .enumerate()
        .map(|(idx, id)| {
            let new_id = format!("moved-{id}");
            let change_key = format!("CQAA{idx:02X}==");
            response_message_xml!("MoveItem", "Success", "NoError", item_success_xml!(new_id, change_key))
        })
        .collect::<String>();

    op_response!("MoveItem", messages)
}

/// Response for batch copy folder operation
pub fn batch_copy_folder_response(folder_ids: &[&str]) -> String {
    let messages = folder_ids
        .iter()
        .enumerate()
        .map(|(idx, id)| {
            let new_id = format!("copied-{id}");
            response_message_xml!(
                "CopyFolder",
                "Success",
                "NoError",
                folder_success_xml!(new_id, format!("AQAA{idx:02X}=="))
            )
        })
        .collect::<String>();

    op_response!("CopyFolder", messages)
}

/// Response for batch move folder operation
pub fn batch_move_folder_response(folder_ids: &[&str]) -> String {
    let messages = folder_ids
        .iter()
        .enumerate()
        .map(|(idx, id)| {
            let new_id = format!("moved-{id}");
            response_message_xml!(
                "MoveFolder",
                "Success",
                "NoError",
                folder_success_xml!(new_id, format!("AQAA{idx:02X}=="))
            )
        })
        .collect::<String>();

    op_response!("MoveFolder", messages)
}

/// Response for batch delete folder operation with mixed success and error
pub fn batch_delete_folder_mixed_response(success_count: usize, error_count: usize) -> String {
    let mut messages = (0..success_count)
        .map(|_| response_message_xml!("DeleteFolder", "Success", "NoError", ""))
        .collect::<String>();

    let error_messages = (0..error_count)
        .map(|_| {
            response_error_xml!(
                "DeleteFolder",
                "ErrorFolderNotFound",
                "The specified object was not found in the store."
            )
        })
        .collect::<String>();

    messages.push_str(&error_messages);

    op_response!("DeleteFolder", messages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_folder_response_contains_folder_id() {
        let response = create_folder_response("test-folder-id");
        assert!(response.contains("test-folder-id"));
        assert!(response.contains("CreateFolderResponse"));
        assert!(response.contains("NoError"));
    }

    #[test]
    fn test_error_response_contains_error_code() {
        let response = error_folder_not_found();
        assert!(response.contains("ErrorFolderNotFound"));
        assert!(response.contains("Error"));
    }

    #[test]
    fn test_soap_structure_is_valid() {
        let response = create_item_response("test-item-id");
        assert!(response.starts_with("<?xml"));
        assert!(response.contains("<s:Envelope"));
        assert!(response.contains("</s:Envelope>"));
    }

    #[test]
    fn test_update_item_response_contains_item_id() {
        let response = update_item_response("test-item-id");
        assert!(response.contains("test-item-id"));
        assert!(response.contains("UpdateItemResponse"));
        assert!(response.contains("ConflictResults"));
    }

    #[test]
    fn test_update_folder_response_contains_folder_id() {
        let response = update_folder_response("test-folder-id");
        assert!(response.contains("test-folder-id"));
        assert!(response.contains("UpdateFolderResponse"));
    }

    #[test]
    fn test_send_item_response_is_valid() {
        let response = send_item_response();
        assert!(response.contains("SendItemResponse"));
        assert!(response.contains("NoError"));
    }

    #[test]
    fn test_find_folder_response_contains_folder_info() {
        let response = find_folder_response("test-folder-id", "TestFolder");
        assert!(response.contains("test-folder-id"));
        assert!(response.contains("TestFolder"));
        assert!(response.contains("FindFolderResponse"));
        assert!(response.contains("RootFolder"));
    }

    #[test]
    fn test_find_item_response_contains_item_id() {
        let response = find_item_response("test-item-id");
        assert!(response.contains("test-item-id"));
        assert!(response.contains("FindItemResponse"));
        assert!(response.contains("RootFolder"));
    }

    #[test]
    fn test_error_invalid_request() {
        let response = error_invalid_request();
        assert!(response.contains("ErrorInvalidRequest"));
        assert!(response.contains("Error"));
    }

    #[test]
    fn test_error_authentication_failed() {
        let response = error_authentication_failed();
        assert!(response.contains("ErrorAuthenticationFailed"));
        assert!(response.contains("Fault"));
    }

    #[test]
    fn test_sync_folder_hierarchy_with_changes() {
        let response = sync_folder_hierarchy_with_changes_response(
            "sync-state-123",
            "created-folder-id",
            "updated-folder-id",
            "deleted-folder-id",
        );
        assert!(response.contains("NewFolder"));
        assert!(response.contains("UpdatedFolder"));
        assert!(response.contains("<t:Create>"));
        assert!(response.contains("<t:Update>"));
        assert!(response.contains("<t:Delete>"));
    }

    #[test]
    fn test_sync_folder_items_with_changes() {
        let response = sync_folder_items_with_changes_response(
            "sync-state-456",
            "created-item-id",
            "updated-item-id",
            "deleted-item-id",
        );
        assert!(response.contains("<t:Create>"));
        assert!(response.contains("<t:Update>"));
        assert!(response.contains("<t:Delete>"));
        assert!(response.contains("<t:ReadFlagChange>"));
        assert!(response.contains("<t:IsRead>true</t:IsRead>"));
    }

    #[test]
    fn test_update_item_set_field() {
        let response = update_item_set_field_response("item-id-set");
        assert!(response.contains("item-id-set"));
        assert!(response.contains("CQAAAB=="));
        assert!(response.contains("UpdateItemResponse"));
    }

    #[test]
    fn test_update_item_append_field() {
        let response = update_item_append_field_response("item-id-append");
        assert!(response.contains("item-id-append"));
        assert!(response.contains("CQAAAC=="));
        assert!(response.contains("UpdateItemResponse"));
    }

    #[test]
    fn test_update_item_delete_field() {
        let response = update_item_delete_field_response("item-id-delete");
        assert!(response.contains("item-id-delete"));
        assert!(response.contains("CQAAAD=="));
        assert!(response.contains("UpdateItemResponse"));
    }

    #[test]
    fn test_error_quota_exceeded() {
        let response = error_quota_exceeded();
        assert!(response.contains("ErrorQuotaExceeded"));
        assert!(response.contains("quota"));
    }

    #[test]
    fn test_error_insufficient_resources() {
        let response = error_insufficient_resources();
        assert!(response.contains("ErrorInsufficientResources"));
        assert!(response.contains("resources"));
    }

    #[test]
    fn test_error_mailbox_store_unavailable() {
        let response = error_mailbox_store_unavailable();
        assert!(response.contains("ErrorMailboxStoreUnavailable"));
        assert!(response.contains("unavailable"));
    }

    // ========================================================================
    // Pagination and Batch Operation Tests
    // ========================================================================

    #[test]
    fn test_find_folder_paginated_response_contains_multiple_folders() {
        let response = find_folder_paginated_response(
            &["folder-1", "folder-2", "folder-3"],
            &["Folder1", "Folder2", "Folder3"],
            3,
            false,
        );
        assert!(response.contains("folder-1"));
        assert!(response.contains("folder-2"));
        assert!(response.contains("folder-3"));
        assert!(response.contains("Folder1"));
        assert!(response.contains("Folder2"));
        assert!(response.contains("Folder3"));
        assert!(response.contains("TotalItemsInView=\"3\""));
        assert!(response.contains("IncludesLastItemInRange=\"false\""));
    }

    #[test]
    fn test_find_item_paginated_response_contains_multiple_items() {
        let response = find_item_paginated_response(&["item-1", "item-2", "item-3"], 10, false);
        assert!(response.contains("item-1"));
        assert!(response.contains("item-2"));
        assert!(response.contains("item-3"));
        assert!(response.contains("TotalItemsInView=\"10\""));
        assert!(response.contains("IncludesLastItemInRange=\"false\""));
    }

    #[test]
    fn test_sync_folder_hierarchy_paginated_response_has_false_includes_last() {
        let response = sync_folder_hierarchy_paginated_response("state-1", "state-2", "folder-id");
        assert!(response.contains("state-2"));
        assert!(response.contains("folder-id"));
        assert!(response.contains("IncludesLastFolderInRange>false"));
    }

    #[test]
    fn test_batch_get_folder_mixed_response_contains_success_and_error() {
        let response = batch_get_folder_mixed_response("success-folder", "error-folder");
        assert!(response.contains("success-folder"));
        assert!(response.contains("SuccessFolder"));
        assert!(response.contains("ResponseClass=\"Success\""));
        assert!(response.contains("ResponseClass=\"Error\""));
        assert!(response.contains("ErrorFolderNotFound"));
    }

    #[test]
    fn test_batch_get_item_response_contains_multiple_items() {
        let response = batch_get_item_response(&["item-1", "item-2"], &["Subject 1", "Subject 2"]);
        assert!(response.contains("item-1"));
        assert!(response.contains("item-2"));
        assert!(response.contains("Subject 1"));
        assert!(response.contains("Subject 2"));
        assert!(response.contains("GetItemResponseMessage"));
    }

    #[test]
    fn test_batch_delete_item_response_contains_multiple_messages() {
        let response = batch_delete_item_response(3);
        let count = response.matches("<m:DeleteItemResponseMessage").count();
        assert_eq!(count, 3);
        assert!(response.contains("NoError"));
    }

    // ========================================================================
    // Additional Error Scenario Tests
    // ========================================================================

    #[test]
    fn test_error_folder_not_empty_response() {
        let response = error_folder_not_empty();
        assert!(response.contains("ErrorFolderNotEmpty"));
        assert!(response.contains("Error"));
        assert!(response.contains("not empty"));
    }

    #[test]
    fn test_error_invalid_change_key_response() {
        let response = error_invalid_change_key();
        assert!(response.contains("ErrorInvalidChangeKey"));
        assert!(response.contains("UpdateItemResponse"));
    }

    #[test]
    fn test_error_invalid_sync_state_response() {
        let response = error_invalid_sync_state();
        assert!(response.contains("ErrorInvalidSyncStateData"));
        assert!(response.contains("SyncFolderHierarchyResponse"));
    }

    // ========================================================================
    // New Fixtures Tests
    // ========================================================================

    #[test]
    fn test_get_folder_full_response_contains_all_fields() {
        let response = get_folder_full_response("folder-id", "parent-id", "TestFolder", "IPF.Note");
        assert!(response.contains("folder-id"));
        assert!(response.contains("parent-id"));
        assert!(response.contains("TestFolder"));
        assert!(response.contains("IPF.Note"));
        assert!(response.contains("ParentFolderId"));
        assert!(response.contains("FolderClass"));
    }

    #[test]
    fn test_get_folder_distinguished_response() {
        let response = get_folder_distinguished_response("inbox", "folder-id-123");
        assert!(response.contains("folder-id-123"));
        assert!(response.contains("inbox"));
        assert!(response.contains("GetFolderResponse"));
    }

    #[test]
    fn test_batch_get_folder_response_contains_multiple_folders() {
        let response = batch_get_folder_response(&["f1", "f2"], &["Folder1", "Folder2"]);
        assert!(response.contains("f1"));
        assert!(response.contains("f2"));
        assert!(response.contains("Folder1"));
        assert!(response.contains("Folder2"));
        assert!(response.contains("IPF.Note"));
    }

    #[test]
    fn test_get_item_with_mime_response_contains_mime_content() {
        let response = get_item_with_mime_response("item-id", "Test Subject", "BASE64_CONTENT");
        assert!(response.contains("item-id"));
        assert!(response.contains("Test Subject"));
        assert!(response.contains("BASE64_CONTENT"));
        assert!(response.contains("MimeContent"));
        assert!(response.contains("DateTimeSent"));
        assert!(response.contains("From"));
    }

    #[test]
    fn test_get_item_full_response_contains_all_details() {
        let response = get_item_full_response(
            "item-id",
            "Subject",
            "John",
            "john@example.com",
            true,
            false,
            2048,
            "High",
            "ref-123",
        );
        assert!(response.contains("item-id"));
        assert!(response.contains("Subject"));
        assert!(response.contains("John"));
        assert!(response.contains("john@example.com"));
        assert!(response.contains("<t:IsRead>true</t:IsRead>"));
        assert!(response.contains("<t:HasAttachments>false</t:HasAttachments>"));
        assert!(response.contains("<t:Size>2048</t:Size>"));
        assert!(response.contains("<t:Importance>High</t:Importance>"));
        assert!(response.contains("<t:References>ref-123</t:References>"));
        assert!(response.contains("InternetMessageId"));
    }

    #[test]
    fn test_get_item_with_recipients_response() {
        let response = get_item_with_recipients_response(
            "item-id",
            "Subject",
            "To Name",
            "to@example.com",
            "Cc Name",
            "cc@example.com",
            "Bcc Name",
            "bcc@example.com",
            "ReplyTo Name",
            "replyto@example.com",
        );
        assert!(response.contains("ToRecipients"));
        assert!(response.contains("CcRecipients"));
        assert!(response.contains("BccRecipients"));
        assert!(response.contains("ReplyTo"));
        assert!(response.contains("To Name"));
        assert!(response.contains("to@example.com"));
        assert!(response.contains("Cc Name"));
        assert!(response.contains("cc@example.com"));
        assert!(response.contains("Bcc Name"));
        assert!(response.contains("bcc@example.com"));
        assert!(response.contains("ReplyTo Name"));
        assert!(response.contains("replyto@example.com"));
    }

    #[test]
    fn test_get_item_with_extended_properties_response() {
        let response = get_item_with_extended_properties_response("item-id", "0x0001", "Test Value");
        assert!(response.contains("ExtendedProperty"));
        assert!(response.contains("ExtendedFieldURI"));
        assert!(response.contains("PropertyTag=\"0x0001\""));
        assert!(response.contains("Test Value"));
    }

    #[test]
    fn test_get_item_with_body_response() {
        let response = get_item_with_body_response("item-id", "HTML", "<html><body>Body</body></html>");
        assert!(response.contains("Body"));
        assert!(response.contains("BodyType=\"HTML\""));
        assert!(response.contains("<html><body>Body</body></html>"));
    }

    #[test]
    fn test_sync_folder_items_paginated_response() {
        let response = sync_folder_items_paginated_response("state-1", "state-2", "item-id");
        assert!(response.contains("state-2"));
        assert!(response.contains("item-id"));
        assert!(response.contains("IncludesLastItemInRange>false"));
    }

    #[test]
    fn test_create_item_send_response() {
        let response = create_item_send_response();
        assert!(response.contains("CreateItemResponse"));
        assert!(response.contains("NoError"));
        assert!(response.contains("Success"));
    }

    #[test]
    fn test_error_mark_as_junk_not_supported() {
        let response = error_mark_as_junk_not_supported();
        assert!(response.contains("MarkAsJunkResponse"));
        assert!(response.contains("ErrorInvalidRequest"));
        assert!(response.contains("not supported"));
        assert!(response.contains("MajorVersion=\"14\""));
        assert!(response.contains("MajorBuildNumber=\"0123\""));
    }

    #[test]
    fn test_sync_folder_hierarchy_contains_parent_and_class() {
        let response = sync_folder_hierarchy_response("sync-state", "folder-id");
        assert!(response.contains("ParentFolderId"));
        assert!(response.contains("FolderClass"));
        assert!(response.contains("IPF.Note"));
    }

    #[test]
    fn test_sync_folder_hierarchy_with_changes_contains_parent_and_class() {
        let response = sync_folder_hierarchy_with_changes_response("state", "c1", "u1", "d1");
        assert!(response.contains("ParentFolderId"));
        assert!(response.contains("FolderClass"));
        assert!(response.contains("IPF.Note"));
    }

    #[test]
    fn test_copy_item_response() {
        let response = copy_item_response("copied-item-id");
        assert!(response.contains("copied-item-id"));
        assert!(response.contains("CopyItemResponse"));
        assert!(response.contains("NoError"));
    }

    #[test]
    fn test_move_item_response() {
        let response = move_item_response("moved-item-id");
        assert!(response.contains("moved-moved-item-id"));
        assert!(response.contains("MoveItemResponse"));
        assert!(response.contains("NoError"));
    }

    #[test]
    fn test_copy_folder_response() {
        let response = copy_folder_response("copied-folder-id");
        assert!(response.contains("copied-folder-id"));
        assert!(response.contains("CopyFolderResponse"));
        assert!(response.contains("NoError"));
    }

    #[test]
    fn test_move_folder_response() {
        let response = move_folder_response("moved-folder-id");
        assert!(response.contains("moved-moved-folder-id"));
        assert!(response.contains("MoveFolderResponse"));
        assert!(response.contains("NoError"));
    }

    #[test]
    fn test_mark_as_junk_response() {
        let response = mark_as_junk_response(Some("junk-item-id"));
        assert!(response.contains("junk-item-id"));
        assert!(response.contains("MarkAsJunkResponse"));
        assert!(response.contains("MovedItemId"));
        assert!(response.contains("ChangeKey"));
    }

    #[test]
    fn test_mark_all_items_as_read_response() {
        let response = mark_all_items_as_read_response();
        assert!(response.contains("MarkAllItemsAsReadResponse"));
        assert!(response.contains("NoError"));
    }

    #[test]
    fn test_batch_get_folder_response_contains_parent_folder_id() {
        let response = batch_get_folder_response(&["f1", "f2"], &["Folder1", "Folder2"]);
        assert!(response.contains("ParentFolderId"));
        assert!(response.contains("root-folder-id"));
    }

    #[test]
    fn test_batch_delete_item_mixed_response() {
        let response = batch_delete_item_mixed_response(2, 1);
        assert!(response.contains("DeleteItemResponse"));
        let success_count = response.matches("ResponseClass=\"Success\"").count();
        let error_count = response.matches("ResponseClass=\"Error\"").count();
        assert_eq!(success_count, 2);
        assert_eq!(error_count, 1);
        assert!(response.contains("ErrorItemNotFound"));
    }

    #[test]
    fn test_batch_update_item_response() {
        let response = batch_update_item_response(&["item-1", "item-2", "item-3"]);
        assert!(response.contains("UpdateItemResponse"));
        assert!(response.contains("item-1"));
        assert!(response.contains("item-2"));
        assert!(response.contains("item-3"));
        let count = response.matches("<m:UpdateItemResponseMessage").count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_batch_update_item_mixed_response() {
        let response = batch_update_item_mixed_response(&["item-1", "item-2"], 1);
        assert!(response.contains("UpdateItemResponse"));
        let success_count = response.matches("ResponseClass=\"Success\"").count();
        let error_count = response.matches("ResponseClass=\"Error\"").count();
        assert_eq!(success_count, 2);
        assert_eq!(error_count, 1);
    }

    #[test]
    fn test_batch_mark_as_junk_response() {
        let response = batch_mark_as_junk_response(&["item-1", "item-2"]);
        assert!(response.contains("MarkAsJunkResponse"));
        assert!(response.contains("item-1"));
        assert!(response.contains("item-2"));
        let count = response.matches("<m:MarkAsJunkResponseMessage").count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_batch_copy_item_response() {
        let response = batch_copy_item_response(&["item-1", "item-2"]);
        assert!(response.contains("CopyItemResponse"));
        assert!(response.contains("copied-item-1"));
        assert!(response.contains("copied-item-2"));
        let count = response.matches("<m:CopyItemResponseMessage").count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_batch_move_item_response() {
        let response = batch_move_item_response(&["item-1", "item-2"]);
        assert!(response.contains("MoveItemResponse"));
        assert!(response.contains("moved-item-1"));
        assert!(response.contains("moved-item-2"));
        let count = response.matches("<m:MoveItemResponseMessage").count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_batch_copy_folder_response() {
        let response = batch_copy_folder_response(&["folder-1", "folder-2"]);
        assert!(response.contains("CopyFolderResponse"));
        assert!(response.contains("copied-folder-1"));
        assert!(response.contains("copied-folder-2"));
        let count = response.matches("<m:CopyFolderResponseMessage").count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_batch_move_folder_response() {
        let response = batch_move_folder_response(&["folder-1", "folder-2"]);
        assert!(response.contains("MoveFolderResponse"));
        assert!(response.contains("moved-folder-1"));
        assert!(response.contains("moved-folder-2"));
        let count = response.matches("<m:MoveFolderResponseMessage").count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_batch_delete_folder_mixed_response() {
        let response = batch_delete_folder_mixed_response(1, 2);
        assert!(response.contains("DeleteFolderResponse"));
        let success_count = response.matches("ResponseClass=\"Success\"").count();
        let error_count = response.matches("ResponseClass=\"Error\"").count();
        assert_eq!(success_count, 1);
        assert_eq!(error_count, 2);
        assert!(response.contains("ErrorFolderNotFound"));
    }
}
