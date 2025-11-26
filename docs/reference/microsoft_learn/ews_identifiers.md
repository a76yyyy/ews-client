# EWS Identifiers in Exchange | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/exchange-web-services/ews-identifiers-in-exchange>

Find out about identifiers in Exchange and how you can use them in your EWS Managed API and EWS applications.

## Overview

Every object in the Exchange store has a unique identifier. You can use an object's identifier to reference the object and to distinguish it from other objects. The two most common identifiers that you might work with are folder and item identifiers.

## Object Hierarchy

When your EWS Managed API or EWS application communicates with Exchange, you work with an object hierarchy that includes mailbox, folder, and item objects. A store can be any one of these object types. Most commonly, it is a mailbox on the Exchange server, but it can also be a public folder on the Exchange server.

> **Note:** In Exchange Online, Exchange Online as part of Office 365, and versions of Exchange starting with Exchange 2013, public folders are just another type of mailbox and not a different kind of store.

The store contains folders and the folders contain items, and each of these folders and items has an identifier.

**Figure 1. Mailbox, folder, and item hierarchy**

The mailbox is at the top level with folders at the next level, and folders contain items. Identifiers and change keys are associated with each object.

## EWS Identifiers

Identifiers that EWS uses for folders and items are called EWS identifiers, or EwsIds. EwsIds can be found in many different objects within EWS, but are called something different for different objects. Because you might use these objects in your application, you'll want to understand how the identifiers for these objects relate to the EwsId.

The identifiers in EWS are applicable to the EWS Managed API as well. In the EWS Managed API, the identifiers are properties of the objects and are internally managed to map to the EWS elements.

### Object Identifiers Table

| **Object** | **Identifier** | **How does it relate to EwsId?** |
|------------|----------------|---------------------------------|
| [CalendarItem](https://msdn.microsoft.com/library/b0c1fd27-b6da-46e5-88b8-88f00c71ba80%28Office.15%29.aspx) | The [ItemId](https://msdn.microsoft.com/library/3350b597-57a0-4961-8f44-8624946719b4%28Office.15%29.aspx) child element contains the unique identifier of the calendar item. | The [ItemId](https://msdn.microsoft.com/library/3350b597-57a0-4961-8f44-8624946719b4%28Office.15%29.aspx) child element is the same as the EwsId for this item. |
| [ConversationId](https://msdn.microsoft.com/library/d5f1ddb3-9af3-4677-a6ba-111b304a951e%28Office.15%29.aspx) | The **Id** attribute contains the identifier for the conversation that this item is part of. | The **Id** attribute is the same as the EwsId for this item. |
| [AttachmentId](https://msdn.microsoft.com/library/55a5fd77-60d1-40fa-8144-770600cedc6a%28Office.15%29.aspx) | Provides the unique identifier of the attachment. The [RootItemId](https://msdn.microsoft.com/library/f613c705-17ce-48ce-aa64-4dc2cea25e31%28Office.15%29.aspx) attribute contains the unique identifier of the root store item to which the attachment is attached. | Attachments can be other items in the Exchange store, in which case the [AttachmentId](https://msdn.microsoft.com/library/55a5fd77-60d1-40fa-8144-770600cedc6a%28Office.15%29.aspx) is the same as the EwsId. In all cases, the [RootItemId](https://msdn.microsoft.com/library/f613c705-17ce-48ce-aa64-4dc2cea25e31%28Office.15%29.aspx) is an EwsId because it references an item in the store. |
| [PersonaId](https://msdn.microsoft.com/library/eec3a468-afd5-4d72-a61e-cd1964fb686c%28Office.15%29.aspx) | The **Id** attribute returns a string that contains the identifier of the persona. | The **Id** attribute is the same as the EwsId for the persona. |
| [ContactId](https://msdn.microsoft.com/library/86f66275-1e39-48ed-bd89-ac3bffc465a7%28Office.15%29.aspx) | The **Id** attribute returns a string that contains the identifier of the contact. | The **Id** attribute is the same as the EwsId for the contact. |
| [GroupId](https://msdn.microsoft.com/library/656d9b9a-8a65-4a75-8466-5b0d96512dab%28Office.15%29.aspx) | The **Id** attribute returns a string that contains the identifier of the group. | The **Id** attribute is the same as the EwsId for the group. |
| [AssociatedCalendarItemId](https://msdn.microsoft.com/library/5b29898c-ea59-4e6a-914c-c011ec754032%28Office.15%29.aspx) | The **Id** attribute identifies the calendar item that is associated with a meeting message. | The **Id** attribute is the same as the EwsId for the calendar item. |
| [UserConfigurationProperties](https://msdn.microsoft.com/library/c143a6ec-62ad-4d48-b844-b1ad88054bc1%28Office.15%29.aspx) | The **Id** value for this element specifies the identifier property. | This identifier does not directly map to the EwsId since it is a property identifier and not an item. |
| [OccurrenceItemId](https://msdn.microsoft.com/library/4a15bbc3-5b93-4193-b9ec-da32f0a9a552%28Office.15%29.aspx) | The **RecurringMasterId** attribute identifies the master of a recurring item. | The **OccurrenceItemId** value does not map directly to the EwsId, but the **RecurringMasterId** does because it references the top-level object of the recurring item. |
| [StoreEntryId](https://msdn.microsoft.com/library/f536e264-8c4d-4cc5-bab8-22a4fa38de39%28Office.15%29.aspx) | Contains the Exchange store identifier of an item. | The **StoreEntryId** value does not map to the EwsId, but it does give the identifier of the store where the items are kept. |

## Working with Identifiers

The Exchange server handles identifiers in a lot of different ways. Consider the following when you develop your EWS Managed API or EWS application:

1. **Case Sensitivity:** The **ItemID** element value for folders and items is case-sensitive. If you look at the item ID for a folder or item that is returned by the [FindItem operation](https://msdn.microsoft.com/library/ebad6aae-16e7-44de-ae63-a95b24539729%28Office.15%29.aspx), you might think that it is a duplicate of another item ID; however, one or more characters in the item IDs for the two items will have a different case.

2. **Storage Size:** If you are going to store the item ID in a database to retrieve later, we recommend that the field size be 512 bytes, so that it's large enough to hold the GUID.

3. **ID Validity:** Don't assume that your item ID will always be valid if you need to retrieve the item at a later time. If an item is moved in the store, the ID can change because of the way a move is handled. An item is actually copied, and a new ID is generated, and then the original item is deleted. Note that folder IDs are immutable, and won't change when moved in the store.

4. **Opaque Identifiers:** Identifiers in Exchange are opaque. For example, the EwsId is created from several pieces of information that are not important to you as the developer, but are important to Exchange.

5. **Change Keys:** When you work with items in Exchange, another value to keep in mind is the **ChangeKey** attribute. The ChangeKey is used to track changes to items and is important for synchronization operations.
