# FolderId | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderid>

The **FolderId** element contains the identifier and change key of a folder.

## Type Information

- **Type Name:** FolderIdType
- **Namespace:** <http://schemas.microsoft.com/exchange/services/2006/types>
- **Schema Name:** Types schema
- **Validation File:** Types.xsd

## Usage

All **FolderId** elements are of the **FolderIdType** type. The **FolderId** element is required in every case except in elements whose type extends the **BaseFolderType** or where the **FolderId** element is a part of a choice. Review the schema for more information.

## Parent Elements

The **FolderId** element is used in the following parent elements:

| **Element** | **Description** |
|-------------|-----------------|
| [ContextFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contextfolderid) | Indicates the folder that is targeted for actions that use folders. |
| [CopiedEvent](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/copiedevent) | Represents an event in which an item or folder is copied. |
| [DestinationFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/destinationfolderid) | Indicates the destination folder for copy and move actions. |
| [ParentFolderId (TargetFolderIdType)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/parentfolderid-targetfolderidtype) | Identifies the folder where a new folder or item is created. |
| [BaseFolderIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/basefolderids) | Represents the collection of folders that will be mined to determine the contents of a search folder. |
| [Delete (FolderSync)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/delete-foldersync) | Identifies a single folder to delete in the local client store. |
| [Folder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folder) | Represents a folder in a mailbox. |
| [CalendarFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/calendarfolder) | Represents a calendar folder in a mailbox. |
| [FolderChange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderchange) | Represents a collection of changes to be performed on a single folder. |
| [ContactsFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contactsfolder) | Represents a contact folder in a mailbox. |
| [SearchFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/searchfolder) | Represents a search folder in a mailbox. |
| [TasksFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/tasksfolder) | Represents a task folder in a mailbox. |
| [ToFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/tofolderid) | Represents the destination folder for a copied or moved item or folder. |
| [SavedItemFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/saveditemfolderid) | Identifies the target folder for operations that update, send, and create items in the Exchange store. |
| [SyncFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderid) | Represents the folder that contains the items to synchronize. |
| [UserConfigurationName](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/userconfigurationname) | Represents the name of a user configuration object. |
| [CopyToFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/copytofolder) | Identifies the ID of the folder that e-mail items will be copied to. |
| [MoveToFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/movetofolder) | Identifies the ID of the folder that e-mail items will be moved to. |

## XPath Expressions

Some elements have specific XPath expressions where the **FolderId** is used:

- `/CreateItem/ParentFolderId`
- `/CreateFolder/ParentFolderId`
- `/UpdateFolder/FolderChanges/FolderChange`
- `/MoveFolder/ToFolderId`
- `/CopyFolder/ToFolderId`
- `/MoveItem/ToFolderId`
- `/CopyItem/ToFolderId`
- `/CreateItem/SavedItemFolderId`
- `/UpdateItem/SavedItemFolderId`
- `/SendItem/SavedItemFolderId`

## Remarks

The schema that describes this element is located in the IIS virtual directory that hosts Exchange Web Services.
