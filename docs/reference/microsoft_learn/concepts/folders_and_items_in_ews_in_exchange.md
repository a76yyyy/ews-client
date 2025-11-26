# Folders and Items in EWS in Exchange

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/folders-and-items-in-ews-in-exchange>

Learn about folders and mailbox items and how your EWS Managed API or EWS client represents them.

Folders are the organizing element of an Exchange mailbox. Folders can contain mailbox items, such as email messages, contacts, appointments, meetings, and tasks, or they can contain other folders. Exchange includes different types of folders, but the folder types are similar to each other. The main difference between them is the type of item they contain.

Items, however, have unique types. Each item type has a different set of properties or schema to define it. In this article, we'll discuss the types of folders and items that are available and the differences between them.

## Folders

Folders all derive from the same base class or base type, the [Folder](https://msdn.microsoft.com/library/microsoft.exchange.webservices.data.folder%28v=EXCHG.80%29.aspx) class in the EWS Managed API, or the [Folder](https://msdn.microsoft.com/library/812948d8-c7db-45ce-bb3a-77233a53a974%28Office.15%29.aspx) type in EWS. The following figure shows the EWS Managed API classes.

**Figure 1. EWS Managed API folder classes**

![An illustration that shows the classes that derive from the EWS Managed API Folder class](https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/media/ex2013_folder_overviewtypes.png)

The primary difference between each of the folder classes is that you can only create a certain type of item in each type of folder. One other difference is in how the client displays information in a folder. For example, Exchange allows you to create appointments in the Calendar folder. You can move other types of items into the Calendar folder after you create them, but Outlook won't display them. Outlook only displays calendar items such as appointments and meetings in the Calendar folder, [even if another type of item exists in the folder](https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/folders-and-items-in-ews-in-exchange#bk_item).

**Table 1. EWS Managed API folder classes**

| EWS Managed API class | FolderClass value | Contains | Notes |
| :--- | :--- | :--- | :--- |
| [Folder](https://msdn.microsoft.com/library/microsoft.exchange.webservices.data.folder%28v=exchg.80%29.aspx) | IPF.Note | Email messages or folders | This is the generic folder class for the following EWS Managed API [WellKnownFolderName](https://msdn.microsoft.com/library/microsoft.exchange.webservices.data.wellknownfoldername%28v=exchg.80%29.aspx) folders: Root (IPM subtree), NonIpmSubtree, Inbox, Deleted Items, Drafts, Journal, Notes, Outbox, Sent Items, Message Folder, Junk Email, Voice Mail |  |
| [CalendarFolder](https://msdn.microsoft.com/library/microsoft.exchange.webservices.data.calendarfolder%28v=exchg.80%29.aspx) | IPF.Appointment | Appointments and meetings | When a user responds to a meeting request, the appointment is added to the EWS Managed API [WellKnownFolderName.Calendar](https://msdn.microsoft.com/library/microsoft.exchange.webservices.data.wellknownfoldername%28v=exchg.80%29.aspx) or the EWS [DistinguishedFolderId.CalendarFolder](https://msdn.microsoft.com/library/50018162-2941-4227-8a5b-d6b4686bb32f%28Office.15%29.aspx) only. These are the only folders that support automatic interaction with meeting requests and responses. |  |
| [ContactsFolder](https://msdn.microsoft.com/library/microsoft.exchange.webservices.data.contactsfolder%28v=exchg.80%29.aspx) | IPF.Contact | Contacts and distribution lists | None. |  |
| [SearchFolder](https://msdn.microsoft.com/library/microsoft.exchange.webservices.data.searchfolder%28v=exchg.80%29.aspx) | IPF.Search | Search folders | None. |  |
| [TasksFolder](https://msdn.microsoft.com/library/microsoft.exchange.webservices.data.tasksfolder%28v=exchg.80%29.aspx) | IPF.Task | Tasks | None. |  |

## Items

Items represent the content of a mailbox. The following table lists the item types that are available in Exchange.

**Table 2. Item types**

| Item type | Description |
| :--- | :--- |
| Message | Email messages, including both mail messages and meeting-related messages |
| Appointment | Calendar appointments and meetings |
| Contact | Personal contacts |
| Task | Tasks |
| Conversation | Conversations |
| Activity | User activities |
| Person | Unified messaging contacts |
| DistributionList | Distribution lists |

## See also

* [Mailbox synchronization and EWS in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/mailbox-synchronization-and-ews-in-exchange)
* [Notification subscriptions, mailbox events, and EWS in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/notification-subscriptions-mailbox-events-and-ews-in-exchange)
