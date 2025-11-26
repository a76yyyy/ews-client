# BaseShape | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape>

The **BaseShape** element identifies the set of properties to return in an item or folder response.

## XML Structure

```xml
<BaseShape>IdOnly or Default or AllProperties</BaseShape>
```

## Type Information

- **Type Name:** DefaultShapeNamesType
- **Namespace:** <http://schemas.microsoft.com/exchange/services/2006/types>
- **Schema Name:** Types schema
- **Validation File:** Types.xsd
- **Can be Empty:** False

## Text Values

A text value is required. The following table lists the possible text values.

| **Value** | **Description** |
|-----------|-----------------|
| **IdOnly** | Returns only the item or folder ID. |
| **Default** | Returns a set of properties that are defined as the default for the item or folder. |
| **AllProperties** | Returns all the properties used by the Exchange Business Logic layer to construct a folder. |

## Default Properties by Folder Type

The following table lists the default properties that are returned for a FindFolder request. All subfolders of a given folder are returned in order by name.

| **Folder** | **Default Properties** |
|------------|------------------------|
| **Inbox** | FolderId, display name, unread count, total count, subfolder count |
| **Contacts** | FolderId, display name, total count, subfolder count |
| **Calendar** | FolderId, display name, subfolder count |
| **Drafts** | FolderId, display name, unread count, total count, subfolder count |
| **Deleted items** | FolderId, display name, unread count, total count, subfolder count |
| **Other folders** | FolderId, display name, unread count, total count, subfolder count |
| **Outbox** | FolderId, display name, unread count, total count, subfolder count |
| **Tasks** | FolderId, display name, past due count, total count, subfolder count |
| **Notes** | FolderId, display name, total count, subfolder count |

## Parent Elements

The **BaseShape** element can be used in the following parent elements:

### FolderShape Element

Used in folder operations:

- `/GetFolder/FolderShape`
- `/FindFolder/FolderShape`
- `/SyncFolderHierarchy/FolderShape`

### ItemShape Element

Used in item operations:

- `/GetItem/ItemShape`
- `/FindItem/ItemShape`
- `/SyncFolderItems/ItemShape`

## Example

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

## Additional Properties

To return properties in addition to those identified by the [BaseShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape) element, use the [AdditionalProperties](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/additionalproperties) element.

## Remarks

The schema that describes this element is located in the IIS virtual directory that hosts Exchange Web Services.

## See Also

- [FolderShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/foldershape)
- [ItemShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemshape)
