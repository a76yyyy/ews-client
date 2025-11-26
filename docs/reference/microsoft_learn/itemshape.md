# ItemShape | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemshape>

The **ItemShape** element identifies a set of properties to return in a [GetItem operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getitem-operation), [FindItem operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation), or [SyncFolderItems operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderitems-operation) response.

## XML Structure

```xml
<ItemShape>
   <BaseShape/>
   <IncludeMimeContent/>
   <BodyType/>
   <FilterHtmlContent/>
   <ConvertHtmlCodePageToUTF8/>
   <AdditionalProperties/>
</ItemShape>
```

## Type Information

- **Type Name:** ItemResponseShapeType
- **Namespace:** <http://schemas.microsoft.com/exchange/services/2006/messages>
- **Schema Name:** Messages schema
- **Validation File:** Messages.xsd
- **Can be Empty:** False

## Child Elements

| **Element** | **Description** |
|-------------|-----------------|
| [BaseShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape) | Identifies the basic configuration of properties to return in an item or folder response. |
| [IncludeMimeContent](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/includemimecontent) | Specifies whether the Multipurpose Internet Mail Extensions (MIME) content of an item is returned in the response. |
| [BodyType](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/bodytype) | Identifies how the body text is formatted in the response. |
| [ConvertHtmlCodePageToUTF8](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/converthtmlcodepagetoutf8) | Indicates whether the item HTML body is converted to UTF8. |
| [FilterHtmlContent](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/filterhtmlcontent) | Specifies whether HTML content filtering is enabled. |
| [AdditionalProperties](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/additionalproperties) | Identifies additional properties to return in a response. |

## Parent Elements

The **ItemShape** element can be used in the following parent elements:

| **Element** | **Description** | **XPath Expression** |
|-------------|-----------------|---------------------|
| [GetItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getitem) | Defines a request to retrieve items from a mailbox in the Exchange store. | `/GetItem` |
| [FindItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem) | Defines a request to find all items that are contained in a folder. | `/FindItem` |
| [SyncFolderItems](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderitems) | Defines a request to synchronize items in an Exchange store folder. | `/SyncFolderItems` |

## Remarks

The schema that describes this element is located in the IIS virtual directory that hosts Exchange Web Services.

## See Also

- [GetItem operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getitem-operation)
- [FindItem operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation)
- [SyncFolderItems operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderitems-operation)
- [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)
