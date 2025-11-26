# BaseItemId | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseitemid>

The **BaseItemId** element represents the base class for IDs that represent items in a mailbox. This is an abstract class and therefore will not occur in an instance document.

## XML Structure

```xml
<BaseItemId/>
```

## Type Information

- **Type Name:** BaseItemIdType
- **Namespace:** <http://schemas.microsoft.com/exchange/services/2006/types>
- **Schema Name:** Types schema
- **Validation File:** Types.xsd
- **Can be Empty:** False

## Description

This element is an abstract base type for item identifiers. It serves as the foundation for all specific item identifier types in Exchange Web Services.

## Attributes and Elements

### Attributes

None.

### Child Elements

None.

### Parent Elements

None.

## Remarks

- This element is an abstract base type for item identifiers.
- This element is not used directly in Web service calls.
- The schema that describes this element is located in the EWS virtual directory of the computer that is running Microsoft Exchange Server 2007 that has the Client Access server role installed.

## See Also

- [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)
