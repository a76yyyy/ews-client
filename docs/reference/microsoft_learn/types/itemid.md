# ItemId | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemid>

The **ItemId** element contains the unique identifier and change key of an item in the Exchange store.

## Type Information

- **Type Name:** ItemIdType
- **Namespace:** <http://schemas.microsoft.com/exchange/services/2006/types>
- **Schema Name:** Types schema
- **Validation File:** Types.xsd

## Parent Elements

The **ItemId** element is used in the following parent elements:

| **Element** | **Description** |
|-------------|-----------------|
| [CalendarItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/calendaritem) | Represents an Exchange calendar item. |
| [Contact](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contact) | Represents an Exchange contact item. |
| [CopiedEvent](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/copiedevent) | Represents an event when an item or folder is copied. |
| [CreatedEvent](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createdevent) | Represents an event when an item or folder is created. |
| [Delete (ItemSync)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/delete-itemsync) | Identifies a single item to delete in the local client store. |
| [DeletedEvent](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deletedevent) | Represents an event when an item or folder is deleted. |
| [DistributionList](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/distributionlist) | Represents a distribution list. |
| [ExportItemsResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/exportitemsresponsemessage) | Contains the status and results of a request to export a single mailbox item. |
| [FirstOccurrence](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/firstoccurrence) | Represents the first occurrence of a recurring calendar item. |
| [GlobalItemIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/globalitemids) | Contains the collection of item identifiers for all conversation items in a mailbox. |
| [Ignore](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ignore) | Identifies items to skip during synchronization. |
| [Item](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/item) | Represents a generic Exchange item. |
| [Item (UploadItemType)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/item-uploaditemtype) | Represents a single item to upload into a mailbox. |
| [ItemChange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchange) | Contains an item identifier and the updates to apply to the item. |
| [ItemIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemids) | Contains the unique identities of items, occurrence items, and recurring master items. |
| [ItemIds (NonEmptyArrayOfItemIdsType)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemids-nonemptyarrayofitemidstype) | Contains an array of item identifiers that identify the items to export from a mailbox. |
| [LastOccurrence](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/lastoccurrence) | Represents the last occurrence of a recurring calendar item. |
| [Mailbox](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailbox) | Identifies a mail-enabled Active Directory directory service object. |
| [MeetingCancellation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingcancellation) | Represents a meeting cancellation in the Exchange store. |
| [MeetingMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingmessage) | Represents a meeting in the Exchange store. |
| [MeetingRequest](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingrequest) | Represents a meeting request in the Exchange store. |
| [MeetingResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingresponse) | Represents a meeting response in the Exchange store. |
| [Message](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref) | Represents an Exchange e-mail message. |
| [ModifiedEvent](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/modifiedevent) | Represents an event that occurs when an item is modified. |
| [MovedEvent](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/movedevent) | Represents an event that occurs when an item is moved from one parent folder to another parent folder. |
| [NewMailEvent](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/newmailevent) | Represents an event that is triggered by a new mail item in a mailbox. |
| [Occurrence](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/occurrence) | Represents a single modified occurrence of a recurring calendar item. |
| [PlayOnPhone (Exchange Web Services)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/playonphone-exchange-web-services) | Represents a request to read an item on a telephone. |
| [RemoveItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/removeitem) | Removes an item from the Exchange store. |
| [RoomList](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/roomlist) | Represents an e-mail address that identifies a list of meeting rooms. |
| [Task](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/task) | Represents a task in the Exchange store. |
| [UploadItemsResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/uploaditemsresponsemessage) | Contains the status and results of a request to upload a single mailbox item. |
| [UserConfiguration](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/userconfiguration) | Defines a single user configuration object. |

## XPath Expressions

Some elements have specific XPath expressions where the **ItemId** is used:

- `/UpdateItem/ItemChanges/ItemChange[i]`
- `/DeleteItem/ItemIds`
- `/SendItem/ItemIds`
- `/GetItem/ItemIds`
- `/MoveItem/ItemIds`
- `/CopyItem/ItemIds`

## Remarks

The schema that describes this element is located in the IIS virtual directory that hosts Exchange Web Services.
