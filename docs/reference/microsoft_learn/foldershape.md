# FolderShape | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/foldershape>

The **FolderShape** element identifies the folder properties to include in a [GetFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getfolder), [FindFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/findfolder), or [SyncFolderHierarchy](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderhierarchy) response.

## XML Structure

```xml
<FolderShape>
   <BaseShape/>
   <AdditionalProperties/>
</FolderShape>
```

## Type Information

- **Type Name:** FolderResponseShapeType
- **Namespace:** <http://schemas.microsoft.com/exchange/services/2006/messages>
- **Schema Name:** Messages schema
- **Validation File:** Messages.xsd
- **Can be Empty:** False

## Child Elements

| **Element** | **Description** |
|-------------|-----------------|
| [BaseShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape) | Identifies the basic configuration of properties to return in a response. |
| [AdditionalProperties](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/additionalproperties) | Identifies additional properties to return in a response. |

## Parent Elements

The **FolderShape** element can be used in the following parent elements:

| **Element** | **Description** | **XPath Expression** |
|-------------|-----------------|---------------------|
| [FindFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/findfolder) | Defines a request to identify folders in a mailbox. | `/FindFolder` |
| [GetFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getfolder) | Defines a request to get a folder from the Exchange store. | `/GetFolder` |
| [SyncFolderHierarchy](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderhierarchy) | Defines a request to synchronize a folder hierarchy on a client. | `/SyncFolderHierarchy` |

## Example

The following example of a request demonstrates how to find all folders located in the first level of the Inbox folder.

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
  xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <FindFolder Traversal="Shallow" xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <FolderShape>
        <t:BaseShape>Default</t:BaseShape>
      </FolderShape>
      <ParentFolderIds>
        <t:DistinguishedFolderId Id="inbox"/>
      </ParentFolderIds>
    </FindFolder>
  </soap:Body>
</soap:Envelope>
```

## Remarks

- The **FolderShape** element is a required child element of the [FindFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/findfolder) element.
- The schema that describes this element is located in the EWS virtual directory of the computer that is running Microsoft Exchange Server 2007 that has the Client Access server role installed.

## See Also

- [FindFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/findfolder)
