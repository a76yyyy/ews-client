# ExtendedProperty | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/extendedproperty>

The **ExtendedProperty** element identifies extended MAPI properties on folders and items.

## XML Structure

The **ExtendedProperty** element can have two different structures depending on whether it represents a single-valued or multi-valued property:

### Single-valued Property

```xml
<ExtendedProperty>
   <ExtendedFieldURI/>
   <Value/>
</ExtendedProperty>
```

### Multi-valued Property

```xml
<ExtendedProperty>
   <ExtendedFieldURI/>
   <Values/>
</ExtendedProperty>
```

## Type Information

- **Type Name:** ExtendedPropertyType
- **Namespace:** <http://schemas.microsoft.com/exchange/services/2006/types>
- **Schema Name:** Types schema
- **Validation File:** Types.xsd
- **Can be Empty:** False

## Child Elements

| **Element** | **Description** |
|-------------|-----------------|
| [ExtendedFieldURI](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/extendedfielduri) | Identifies an extended MAPI property to get, set, or create. |
| [Values](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/values) | Contains a collection of values for a multivalued extended MAPI property. |
| [Value](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/value) | Contains the value of single-valued MAPI extended property. |

## Parent Elements

The **ExtendedProperty** element can be used in the following parent elements:

### Item Types

- [CalendarItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/calendaritem) - Represents an Exchange calendar item.
- [Contact](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contact) - Represents an Exchange contact item.
- [DistributionList](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/distributionlist) - Represents a distribution list.
- [Item](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/item) - Represents an item in the Exchange store.
- [MeetingCancellation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingcancellation) - Represents a meeting cancellation in the Exchange store.
- [MeetingMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingmessage) - Represents a meeting in the Exchange store.
- [MeetingRequest](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingrequest) - Represents a meeting request in the Exchange store.
- [MeetingResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingresponse) - Represents a meeting response in the Exchange store.
- [Message](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref) - Represents an Exchange e-mail message.
- [RemoveItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/removeitem) - Removes an item from the Exchange store.
- [Task](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/task) - Represents a task in the Exchange store.

### Folder Types

- [CalendarFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/calendarfolder) - Represents a folder that primarily contains calendar items.
- [ContactsFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contactsfolder) - Represents a contacts folder in a mailbox.
- [Folder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folder) - Represents a folder to create, get, find, synchronize, or update.
- [SearchFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/searchfolder) - Represents a search folder that is contained in a mailbox.
- [TasksFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/tasksfolder) - Represents a tasks folder that is contained in a mailbox.

## Remarks

The schema that describes this element is located in the EWS virtual directory of the computer that is running Microsoft Exchange Server 2007 that has the Client Access server role installed.

## See Also

- [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)
