# FindFolder operation | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/findfolder-operation>

The **FindFolder** operation uses Exchange Web Services to find subfolders of an identified folder and returns a set of properties that describe the set of subfolders.

FindFolder returns only the first 512 bytes of any streamable property. For Unicode, it returns the first 255 characters by using a null-terminated Unicode string.

Deep traversal queries cannot be performed on public folders.

Restrictions are permitted and should use only the folder properties, not the item properties. Sorting functionality is not available for **FindFolder** responses. Grouped queries are not available for **FindFolder** queries.

> [!NOTE]
> The **FindFolder** operation is also used to find managed folders.

The **FindFolder** operation can use the SOAP headers that are listed and described in the following table.

| **Header** | **Element** | **Description** |
|------------|-------------|-----------------|
| Impersonation | [ExchangeImpersonation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/exchangeimpersonation) | Identifies the user whom the client application is impersonating. |
| MailboxCulture | [MailboxCulture](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailboxculture) | Identifies the RFC3066 culture to be used to access the mailbox. |
| RequestVersion | [RequestServerVersion](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/requestserverversion) | Identifies the schema version for the operation request. |
| ServerVersion | [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo) | Identifies the version of the server that responded to the request. |
| TimeZoneContext | [TimeZoneContext](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/timezonecontext) | Identifies the time zone to be used for all responses from the server. |

## FindFolder request example

The following example of a **FindFolder** request shows how to form a request to find all the folders located in an Inbox.

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

Using the Default value for the [BaseShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape), the response returns the folder name, the folder ID, the number of subfolders, the number of child folders found in the folder, and the count of unread items.

### FindFolder request elements

This **FindFolder** request includes the following elements:

* [FindFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/findfolder)
* [FolderShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/foldershape)
* [BaseShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape)
* [ParentFolderIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/parentfolderids)
* [DistinguishedFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/distinguishedfolderid)

For additional **FindFolder** request elements, see the schema.

## FindFolder response example

The following Simple Object Access Protocol (SOAP) body example shows a successful response to the **FindFolder** request. The response contains the elements that are returned when the Default value for the [BaseShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape) is used.

> **Note**
>
> The folder ID and the change key have been shortened to preserve readability.

```xml
<?xml version="1.0" encoding="utf-8" ?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="652" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <FindFolderResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                        xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                        xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:FindFolderResponseMessage ResponseClass="Success">
          <m:ResponseCode>NoError</m:ResponseCode>
          <m:RootFolder TotalItemsInView="1" IncludesLastItemInRange="true">
            <t:Folders>
              <t:Folder>
                <t:FolderId Id="AQAnAH" ChangeKey="AQAAABY" />
                <t:DisplayName>TestFolder</t:DisplayName>
                <t:TotalCount>0</t:TotalCount>
                <t:ChildFolderCount>0</t:ChildFolderCount>
                <t:UnreadCount>0</t:UnreadCount>
              </t:Folder>
            </t:Folders>
          </m:RootFolder>
        </m:FindFolderResponseMessage>
      </m:ResponseMessages>
    </FindFolderResponse>
  </soap:Body>
</soap:Envelope>
```

### FindFolder response elements

The properties that are returned in the response are determined by the [BaseShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape) and the [AdditionalProperties](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/additionalproperties) if they are used. A successful **FindFolder** response includes the following elements:

* [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo)
* [FindFolderResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/findfolderresponse)
* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)
* [FindFolderResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/findfolderresponsemessage)
* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)
* [RootFolder (FindItemResponseMessage)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/rootfolder-finditemresponsemessage)
* [Folders](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folders-ex15websvcsotherref)
* [Folder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folder)
* [FolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderid)
* [DisplayName (string)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/displayname-string)
* [TotalCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/totalcount)
* [ChildFolderCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/childfoldercount)
* [UnreadCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/unreadcount)

**FindFolder** responses to a request with the **AllProperties** response shape will not return the [TotalCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/totalcount) and [UnreadCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/unreadcount) elements for public folder searches.

## FindFolder Error response example

The following SOAP body example shows an error response that occurs when you search for a folder that is identified by a malformed folder identifier.

```xml
<?xml version="1.0" encoding="utf-8" ?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
                 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                 xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="652" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/en-us/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <FindFolderResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                          xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                          xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:FindFolderResponseMessage ResponseClass="Error">
          <m:MessageText>Id is malformed.</m:MessageText>
          <m:ResponseCode>ErrorInvalidIdMalformed</m:ResponseCode>
          <m:DescriptiveLinkKey>0</m:DescriptiveLinkKey>
        </m:FindFolderResponseMessage>
      </m:ResponseMessages>
    </FindFolderResponse>
  </soap:Body>
</soap:Envelope>
```

### FindFolder error response elements

The **FindFolder** error response includes the following elements:

* [FindFolderResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/findfolderresponse)
* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)
* [FindFolderResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/findfolderresponsemessage)
* [MessageText](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/messagetext)
* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)
* [DescriptiveLinkKey](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/descriptivelinkkey)
