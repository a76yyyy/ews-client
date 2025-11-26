# Recipients (ArrayOfRecipientsType) | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/recipients-arrayofrecipientstype>

The **Recipients** element represents a collection of recipients that receive a copy of the message.

## XML Structure

```xml
<Recipients>
   <Mailbox/>
</Recipients>
```

## Type Information

- **Type Name:** ArrayOfRecipientsType
- **Namespace:** <http://schemas.microsoft.com/exchange/services/2006/types>
- **Schema Name:** Types schema
- **Validation File:** Types.xsd
- **Can be Empty:** False

## Child Elements

| **Element** | **Description** |
|-------------|-----------------|
| [Mailbox](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailbox) | Identifies a mail-enabled Active Directory object. |

## Parent Elements

| **Element** | **Description** |
|-------------|-----------------|
| [GetMailTips](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getmailtips) | Contains the recipients and types of mail tips to retrieve. |

## Text Value

None.

## Remarks

The schema that describes this element is located in the IIS virtual directory that hosts Exchange Web Services.

## See Also

- [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)
