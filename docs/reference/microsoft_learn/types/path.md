# Path | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/path>

The **Path** element is the base schema type for all property identifiers in Exchange Web Services (EWS). This type is abstract and will never occur directly within instance documents.

## XML Structure

```xml
<Path/>
```

## Type Information

- **Type Name:** BasePathToElementType
- **Namespace:** <http://schemas.microsoft.com/exchange/services/2006/types>
- **Schema Name:** Types schema
- **Validation File:** Types.xsd
- **Can be Empty:** False

## Description

The **Path** element serves as an abstract base type for all property path elements in EWS. It cannot be used directly in XML documents but is substituted by specific path elements.

## Substituting Elements

The following elements are used to substitute for the **Path** element:

- [FieldURI](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/fielduri) - Used for standard property field identifiers
- [IndexedFieldURI](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/indexedfielduri) - Used for indexed property field identifiers
- [ExceptionFieldURI](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/exceptionfielduri) - Used for exception property field identifiers
- [ExtendedFieldURI](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/extendedfielduri) - Used for extended MAPI property field identifiers

## Attributes and Elements

### Attributes

None.

### Child Elements

None.

## Remarks

The schema that describes this element is located in the EWS virtual directory of the computer that is running Microsoft Exchange Server 2007 that has the Client Access server role installed.

## See Also

- [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)
