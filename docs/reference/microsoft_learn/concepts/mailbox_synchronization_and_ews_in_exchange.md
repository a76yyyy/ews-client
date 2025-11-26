# Mailbox Synchronization and EWS in Exchange

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/mailbox-synchronization-and-ews-in-exchange>

Find out how mailbox synchronization works when you use EWS to access Exchange.

EWS in Exchange uses two types of synchronization to retrieve mailbox content and changes to mailbox content:

* Folder synchronization
* Item synchronization

In this article, you'll learn about both types of synchronization, how synchronization works, synchronization design patterns, and synchronization best practices.

## Folder and item synchronization

Folder synchronization syncs a folder structure, or folder hierarchy. Item synchronization syncs the items within a folder. When you synchronize items, you have to sync each folder in the mailbox independently. You can use EWS or the EWS Managed API in your application to implement both folder and item synchronization.

**Table 1. EWS operations and EWS Managed API methods for syncing folders and items**

| EWS Operation | EWS Managed API Method | Description |
| :--- | :--- | :--- |
| SyncFolderHierarchy | ExchangeService.SyncFolderHierarchy | Synchronizes the local folder hierarchy with the server. |
| SyncFolderItems | ExchangeService.SyncFolderItems | Synchronizes items in a specific folder. |

The scope of the synchronization that occurs differs depending on whether it is an initial or an ongoing sync, as follows:

* An initial synchronization syncs all folders or items on the server to the client. After the initial synchronization, the client has a sync state that it stores for future synchronizations. The sync state represents all the changes on the server that the server communicated to the client.
* Ongoing synchronizations sync any items or folders that have been added, deleted, or changed since the previous synchronization. The server uses the sync state to calculate the changes to report to the client during each of the ongoing synchronization loops.

Each synchronization method or operation returns a list of changes, not the actual folder or message that changed. Changes to items and folders are reported by means of the following change types:

* Create — Indicates that a new item or folder should be created on the client.
* Update — Indicates that an item or folder should be changed on the client.
* Delete — Indicates that an item or folder should be deleted on the client.
* ReadStateChange for EWS or ReadFlagChange for the EWS Managed API — Indicates that that the read state of the item has changed, either from unread to read, or read to unread.

In Exchange Online, Exchange Online as part of Office 365, and versions of Exchange starting with Exchange 2010 SP2, items and folders are returned in order from newest to oldest. In previous versions of Exchange, items and folders are returned from oldest to newest.

## How does EWS synchronization work?

In a nutshell, if you're synchronizing a mailbox for the first time, use the process as shown in Figure 1. Although you can use other [synchronization design patterns](https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/mailbox-synchronization-and-ews-in-exchange#bk_syncpatterns), we recommend this approach for scalable applications.

**Figure 1. Initial synchronization design pattern**

![An illustration that shows the initial synchronization design pattern. The client calls SyncFolderHierarchy and Load or GetItem to get the folders, then calls SyncFolderItems and LoadPropertiesForItems or GetItem to get the items in each folder.](https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/media/exchange2013_syncinitial.png)

If you're using an existing sync state on the client to synchronize a mailbox, we recommend that you implement the design pattern as shown in Figure 2.

**Figure 2. Ongoing synchronization design pattern**

![An illustration that shows the ongoing synchronization design pattern. A client receives a notification and then calls SyncFolderHierarchy or SyncFolderItems, gets the properties, then updates the client, or simply updates the read flag on the client.](https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/media/exchange2013_sync_ongoing.png)

## Synchronization design patterns

You can use one of two synchronization design patterns in your application to keep your mailboxes up to date: notification-based synchronization, or the synchronization-only approach.

Notification-based synchronization, as illustrated in [Figure 2](https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/mailbox-synchronization-and-ews-in-exchange#bk_howdoesitwork), relies on notifications to alert the client to make a call to the EWS Managed API [SyncFolderItems](https://msdn.microsoft.com/library/microsoft.exchange.webservices.data.exchangeservice.syncfolderitems%28v=exchg.80%29.aspx) or [SyncFolderHierarchy](https://msdn.microsoft.com/library/microsoft.exchange.webservices.data.exchangeservice.syncfolderhierarchy%28v=exchg.80%29.aspx) methods, or the EWS [SyncFolderHierarchy](https://msdn.microsoft.com.com/library/b31916b1-bc6c-4451-a475-b7c5417f752d%28Office.15%29.aspx) or [SyncFolderItems](https://msdn.microsoft.com.com/library/7f0de089-8876-47ec-a871-df118ceae75d%28Office.15%29.aspx) operations. This t
