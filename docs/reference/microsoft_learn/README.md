# Microsoft Learn Documentation Archive

This directory contains archived Microsoft Learn documentation for EWS (Exchange Web Services) that was crawled from Microsoft's official documentation.

## Purpose

These documents provide comprehensive information about EWS operations, best practices, and implementation details that can be useful for developers working with the ews-client project.

## Documents Available

### EWS Concepts & Best Practices

* [Mailbox Synchronization and EWS](concepts/mailbox_synchronization_and_ews_in_exchange.md) - Synchronization patterns
* [Synchronization Best Practices](concepts/synchronization_best_practices.md) - Performance and reliability recommendations
* [Folders and Items in EWS](concepts/folders_and_items_in_ews_in_exchange.md) - Folder and item types explanation
* [SyncFolderItems operation](concepts/how_to_synchronize_items.md) - Synchronizing items within folders

### EWS Operations

#### Item Operations

* [CreateItem operation](operations/create_item_operation.md) - Creating email messages and other items
* [DeleteItem operation](operations/deleteitem_operation.md) - Deleting items
* [UpdateItem operation](operations/update_item_operation.md) - Updating existing items
* [GetItem operation](operations/getitem_operation.md) - Retrieving individual items
* [FindItem operation](operations/finditem_operation.md) - Searching for items
* [MoveItem operation](operations/moveitem_operation.md) - Moving items between folders
* [CopyItem operation](operations/copyitem_operation.md) - Copying items
* [SendItem operation](operations/senditem_operation.md) - Sending messages
* [MarkAllItemsAsRead operation](operations/mark_all_items_as_read_operation.md) - Marking items as read/unread

#### Folder Operations

* [FindFolder operation](operations/findfolder_operation.md) - Searching for folders
* [GetFolder operation](operations/getfolder_operation.md) - Retrieving folder properties
* [SyncFolderHierarchy operation](operations/sync_folder_hierarchy_operation.md) - Synchronizing folder hierarchy

### EWS Types and Elements

#### Identifiers

* [Identifiers in Exchange](types/ews_identifiers.md) - Overview of EWS identifiers
* [FolderId](types/folderid.md) - Folder identifiers
* [ItemId](types/itemid.md) - Item identifiers
* [BaseItemId](types/baseitemid.md) - Base item identifiers
* [DistinguishedFolderId](types/distinguished_folder_id.md) - Default folder identifiers

#### Properties & Shapes

* [ItemShape](types/itemshape.md) - Item property shapes
* [FolderShape](types/foldershape.md) - Folder property shapes
* [BaseShape](types/baseshape.md) - Base property shapes
* [FieldURI](types/fielduri.md) - Property field identifiers
* [Path](types/path.md) - Property paths
* [ExtendedProperty](types/extendedproperty.md) - Extended properties

#### Content & Collections

* [MimeContent](types/mimecontent.md) - MIME message content
* [ArrayOfRecipients](types/recipients_arrayofrecipientstype.md) - Recipient collections
* [UpdateItem Element](types/updateitem.md) - UpdateItem XML element structure

#### Responses

* [ResponseCode](responses/responsecode.md) - Response status codes

### MAPI Properties

* [PidTagMessageFlags](mapi/pidtagmessageflags_canonical_property.md) - Message flags
* [PidTagHasAttachments](mapi/pidtaghasattachments_canonical_property.md) - Attachment flags
* [Property Types](mapi/property_types.md) - MAPI data types

## Usage

These documents can be used for:

* Understanding EWS operation details and parameters
* Implementing similar functionality in the ews-client project
* Reference for API design and best practices
* Learning about Exchange Server capabilities

## Disclaimer

These documents are archived from Microsoft Learn and may not reflect the most current information. For the latest documentation, please visit the official Microsoft Learn website.
