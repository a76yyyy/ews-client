# MimeContent | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mimecontent>

The **MimeContent** element contains the ASCII MIME stream of an object that is represented in base64Binary format and supports [RFC2045](http://www.rfc-editor.org/rfc/rfc2045.txt).

## XML Structure

```xml
<MimeContent CharacterSet="" />
```

## Type Information

- **Type Name:** MimeContentType
- **Namespace:** <http://schemas.microsoft.com/exchange/services/2006/types>
- **Schema Name:** Types schema
- **Validation File:** Types.xsd
- **Can be Empty:** False

## Attributes

| **Attribute** | **Description** |
|---------------|-----------------|
| **CharacterSet** | If set, the value for this attribute is ignored by the server. |

## Child Elements

None.

## Parent Elements

The **MimeContent** element can be used in the following parent elements:

- [CalendarItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/calendaritem)
- [Contact](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contact)
- [DistributionList](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/distributionlist)
- [Item](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/item)
- [MeetingCancellation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingcancellation)
- [MeetingMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingmessage)
- [MeetingRequest](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingrequest)
- [MeetingResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingresponse)
- [Message](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref)
- [RemoveItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/removeitem)
- [Task](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/task)

## Text Value

A text value that represents a base64Binary MIME stream is required if this element is used.

## Encoding Levels

The message content goes through the following three levels of encoding before it is stored in the **MimeContent** value:

1. **Message text** - This is the body encoding, such as iso-2022-jp for Japanese characters.
2. **MIME stream** - This is the ASCII encoding of the message text for the **MimeContent** element, or the UTF8 encoding of the message text for the [MimeContentUTF8](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mimecontentutf8) element.
3. **XML document** - This is always the base64-encoded ASCII stream of the MIME stream, where characters such as '<', which are meaningful to XML, are hidden from XML parsers.

Each level is independent of the level that precedes it.

## Remarks

- The **MimeContent** element might contain the same data that other properties that are returned with an item contain.
- The schema that describes this element is located in the IIS virtual directory that hosts Exchange Web Services.

## See Also

- [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)
