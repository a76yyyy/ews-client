# DistinguishedFolderId

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/distinguishedfolderid>

The DistinguishedFolderId element identifies folders that are associated with a specific account. Each distinguished folder represents a default folder in the mailbox.

## Distinguished folder types

The following table lists the distinguished folder types that are available in Exchange.

| Distinguished folder type | Description |
| :--- | :--- |
| calendar | Represents the Calendar folder. |
| contacts | Represents the Contacts folder. |
| deleteditems | Represents the Deleted Items folder. |
| drafts | Represents the Drafts folder. |
| inbox | Represents the Inbox folder. |
| journal | Represents the Journal folder. |
| notes | Represents the Notes folder. |
| outbox | Represents the Outbox folder. |
| sentitems | Represents the Sent Items folder. |
| tasks | Represents the Tasks folder. |
| msgfolderroot | Represents the message folder root. |
| root | Represents the root of the mailbox. |
| junkemail | Represents the Junk Email folder. |
| searchfolders | Represents the Search Folders folder. |
| voicemail | Represents the Voice Mail folder. |
| recoverableitemsroot | Represents the dumpster root folder. |
| recoverableitemsdeletions | Represents the dumpster deletions folder. |
| recoverableitemsversions | Represents the dumpster versions folder. |
| recoverableitemspurges | Represents the dumpster purges folder. |
| archiveroot | Represents the root archive folder. |
| archivemsgfolderroot | Represents the root archive message folder. |
| archivedeleteditems | Represents the archive deleted items folder. |
| archiveinbox | Represents the archive Inbox folder. |
| archiverecoverableitemsroot | Represents the archive recoverable items root folder. |
| archiverecoverableitemsdeletions | Represents the archive recoverable items deletions folder. |
| archiverecoverableitemsversions | Represents the archive recoverable items versions folder. |
| archiverecoverableitemspurges | Represents the archive recoverable items purges folder. |
| syncissues | Represents the sync issues folder. |
| conflicts | Represents the conflicts folder. |
| localfailures | Represents the local failures folder. |
| serverfailures | Represents the server failures folder. |
| recipientcache | Represents the recipient cache folder. |
| quickcontacts | Represents the quick contacts folder. |
| conversationhistory | Represents the conversation history folder. |
| adminauditlogs | Represents the admin audit logs folder. |
| todosearch | Represents the todo search folder. |
| mycontacts | Represents the My Contacts folder. |
| directory | Represents the directory folder. |
| imcontactlist | Represents the IM contact list folder. |
| peopleconnect | Represents the people connect folder. |
| favorites | Represents the Favorites folder. |

## Usage

The DistinguishedFolderId element is used in several EWS operations to identify the folder to use for the operation. For example:

* In the [SyncFolderHierarchy](https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/reference/syncfolderhierarchy-operation) operation, it identifies the root folder to synchronize.
* In the [FindFolder](https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/reference/findfolder-operation) operation, it identifies folders to search in.
* In the [CreateFolder](https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/reference/createfolder-operation) operation, it identifies the parent folder.

## Remarks

The DistinguishedFolderId element is required for the following operations:

* CreateFolder
* FindFolder
* GetFolder
* SyncFolderHierarchy

For more information about using distinguished folders, see the specific operation documentation.
