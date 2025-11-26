# Synchronization Best Practices | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/mailbox-synchronization-and-ews-in-exchange#bk_bestpractices>

Find out how mailbox synchronization works when you use EWS to access Exchange.

## Overview

EWS in Exchange uses two types of synchronization to retrieve mailbox content and changes to mailbox content:

- **Folder synchronization** - syncs a folder structure, or folder hierarchy
- **Item synchronization** - syncs the items within a folder

When you synchronize items, you have to sync each folder in the mailbox independently. You can use EWS or the EWS Managed API in your application to implement both folder and item synchronization.

## Synchronization Types

### Initial vs Ongoing Synchronization

The scope of the synchronization that occurs differs depending on whether it is an initial or an ongoing sync:

- **Initial synchronization** - syncs all folders or items on the server to the client. After the initial synchronization, the client has a sync state that it stores for future synchronizations. The sync state represents all the changes on the server that the server communicated to the client.

- **Ongoing synchronizations** - sync any items or folders that have been added, deleted, or changed since the previous synchronization. The server uses the sync state to calculate the changes to report to the client during each of the ongoing synchronization loops.

## Change Types

Each synchronization method or operation returns a list of changes, not the actual folder or message that changed. Changes to items and folders are reported by means of the following change types:

- **Create** - Indicates that a new item or folder should be created on the client.
- **Update** - Indicates that an item or folder should be changed on the client.
- **Delete** - Indicates that an item or folder should be deleted on the client.
- **ReadStateChange** (for EWS) or **ReadFlagChange** (for the EWS Managed API) - Indicates that the read state of the item has changed, either from unread to read, or read to unread.

## Order of Results

In Exchange Online, Exchange Online as part of Office 365, and versions of Exchange starting with Exchange 2010 SP2, items and folders are returned in order from newest to oldest. In previous versions of Exchange, items and folders are returned from oldest to newest.

## Synchronization Design Patterns

### Initial Synchronization Pattern

For the first-time synchronization of a mailbox, use the process as shown in Figure 1. Although you can use other synchronization design patterns, this approach is recommended for scalable applications.

**Figure 1. Initial synchronization design pattern**

The client calls:

1. **SyncFolderHierarchy** and **Load** or **GetItem** to get the folders
2. **SyncFolderItems** and **LoadPropertiesForItems** or **GetItem** to get the items in each folder

### Ongoing Synchronization Pattern

If you're using an existing sync state on the client to synchronize a mailbox, implement the design pattern as shown in Figure 2.

**Figure 2. Ongoing synchronization design pattern**

A client receives a notification and then:

1. Calls **SyncFolderHierarchy** or **SyncFolderItems**
2. Gets the properties
3. Updates the client, or simply updates the read flag on the client

## Notification-Based vs Synchronization-Only Approach

### Notification-Based Synchronization

This approach relies on notifications to alert the client to make a call to the EWS Managed API **SyncFolderItems** or **SyncFolderHierarchy** methods, or the EWS **SyncFolderHierarchy** or **SyncFolderItems** operations.

**Advantages:**

- Notifications are optimized to reduce calls to the backend Exchange database
- Event queues and subscriptions are managed by the mailbox server
- Uses fewer resources than frequent synchronization calls
- Exchange has specific throttling policies for notifications and subscriptions

**Drawbacks:**

- Notifications are noisy because most scenarios involve multiple notifications for one user intent
- Calendar folder notifications are especially noisy (e.g., multiple notifications for a single meeting request)
- Notifications are queued on the mailbox server
- If the mailbox server managing the subscription is unavailable, you lose notifications
- Requires mitigation strategies for notification failures

**Mitigation Strategy:**
Build a delay of a few seconds into your **Load**, **LoadPropertiesForItems**, **GetItem**, or **GetFolder** calls to batch multiple changes together.

### Synchronization-Only Approach

This approach is more resilient than notification-based synchronization because:

- Only requires that the client maintain the sync state
- No issues with affinity to the mailbox server managing the subscription
- More reliable in case of server failures

## Best Practices Summary

1. **For scalable applications**, use notification-based synchronization with appropriate delays to handle noisy notifications.

2. **For reliability**, consider the synchronization-only approach, especially if notification failures are a concern.

3. **Always maintain sync state** on the client for ongoing synchronizations.

4. **Handle multiple change types** appropriately (Create, Update, Delete, ReadStateChange).

5. **Consider Exchange version differences** in result ordering (newest to oldest vs oldest to newest).

6. **Implement proper error handling** for both notification failures and synchronization failures.

7. **Use appropriate throttling** to avoid overwhelming the Exchange server with synchronization requests.
