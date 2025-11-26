# Microsoft Learn Documentation Archive

This directory contains archived Microsoft Learn documentation for EWS (Exchange Web Services) that was crawled from Microsoft's official documentation.

## Purpose

These documents provide comprehensive information about EWS operations, best practices, and implementation details that can be useful for developers working with the ews-client project.

## Documents Available

### EWS Operations

#### Item Operations

* [CreateItem operation](create_item_operation.md) - Creating email messages and other items
* [DeleteItem operation](deleteitem_operation.md) - Deleting items
* [UpdateItem operation](update_item_operation.md) - Updating existing items
* [GetItem operation](getitem_operation.md) - Retrieving individual items
* [FindItem operation](finditem_operation.md) - Searching for items
* [MoveItem operation](moveitem_operation.md) - Moving items between folders
* [CopyItem operation](copyitem_operation.md) - Copying items
* [SendItem operation](senditem_operation.md) - Sending messages
* [MarkAllItemsAsRead operation](mark_all_items_as_read_operation.md) - Marking items as read/unread

#### Folder Operations

* [FindFolder operation](findfolder_operation.md) - Searching for folders
* [GetFolder operation](getfolder_operation.md) - Retrieving folder properties
* [SyncFolderHierarchy operation](sync_folder_hierarchy_operation.md) - Synchronizing folder hierarchy
* [SyncFolderItems operation](how_to_synchronize_items.md) - Synchronizing items within folders

### EWS Types and Elements

#### Identifiers

* [Identifiers in Exchange](ews_identifiers.md) - Overview of EWS identifiers
* [FolderId](folderid.md) - Folder identifiers
* [ItemId](itemid.md) - Item identifiers
* [BaseItemId](baseitemid.md) - Base item identifiers
* [DistinguishedFolderId](distinguished_folder_id.md) - Default folder identifiers

#### Properties & Shapes

* [ItemShape](itemshape.md) - Item property shapes
* [FolderShape](foldershape.md) - Folder property shapes
* [BaseShape](baseshape.md) - Base property shapes
* [FieldURI](fielduri.md) - Property field identifiers
* [Path](path.md) - Property paths
* [ExtendedProperty](extendedproperty.md) - Extended properties

#### Content & Collections

* [MimeContent](mimecontent.md) - MIME message content
* [ArrayOfRecipients](recipients_arrayofrecipientstype.md) - Recipient collections
* [UpdateItem Element](updateitem.md) - UpdateItem XML element structure

#### Responses

* [ResponseCode](responsecode.md) - Response status codes

### EWS Concepts & Best Practices

* [Mailbox Synchronization and EWS](mailbox_synchronization_and_ews_in_exchange.md) - Synchronization patterns
* [Synchronization Best Practices](synchronization_best_practices.md) - Performance and reliability recommendations
* [Folders and Items in EWS](folders_and_items_in_ews_in_exchange.md) - Folder and item types explanation

### MAPI Properties

* [PidTagMessageFlags](pidtagmessageflags_canonical_property.md) - Message flags
* [PidTagHasAttachments](pidtaghasattachments_canonical_property.md) - Attachment flags
* [Property Types](property_types.md) - MAPI data types

## Usage

These documents can be used for:

* Understanding EWS operation details and parameters
* Implementing similar functionality in the ews-client project
* Reference for API design and best practices
* Learning about Exchange Server capabilities

## Disclaimer

These documents are archived from Microsoft Learn and may not reflect the most current information. For the latest documentation, please visit the official Microsoft Learn website.
