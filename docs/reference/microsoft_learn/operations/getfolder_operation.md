# GetFolder operation | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getfolder-operation>

The **GetFolder** operation gets folders from the Exchange store.

## GetFolder request example

### Description

The following example of a **GetFolder** request shows how to obtain a folder identifier, display name, the count of items in that folder, the count of child folders, and the number of unread items in the folder.

### Code

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
   xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <GetFolder xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"
               xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
      <FolderShape>
        <t:BaseShape>Default</t:BaseShape>
      </FolderShape>
      <FolderIds>
        <t:DistinguishedFolderId Id="inbox"/>
      </FolderIds>
    </GetFolder>
  </soap:Body>
</soap:Envelope>
```

### Request elements

This **GetFolder** request includes the following elements:

* [GetFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getfolder)

* [FolderShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/foldershape)

* [BaseShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape)

* [FolderIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderids)

* [DistinguishedFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/distinguishedfolderid)

See the schema for additional elements that you can use to form a **GetFolder** request.

> **Note**
>
> The schema that describes this element is located in the IIS virtual directory that hosts Exchange Web Services.

## GetFolder response example

### Description

The following Simple Object Access Protocol (SOAP) body example shows a successful response to the **GetFolder** request.

> **Note**
>
> The folder ID and the change key have been shortened to preserve readability.

### Code

```xml
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="628" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <GetFolderResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                       xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                       xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:GetFolderResponseMessage ResponseClass="Success">
          <m:ResponseCode>NoError</m:ResponseCode>
          <m:Folders>
            <t:Folder>
              <t:FolderId Id="AQApA=" ChangeKey="AQAAAB" />
              <t:DisplayName>Inbox</t:DisplayName>
              <t:TotalCount>2</t:TotalCount>
              <t:ChildFolderCount>0</t:ChildFolderCount>
              <t:UnreadCount>2</t:UnreadCount>
            </t:Folder>
          </m:Folders>
        </m:GetFolderResponseMessage>
      </m:ResponseMessages>
    </GetFolderResponse>
  </soap:Body>
</soap:Envelope>
```

### Response elements

This **GetFolder** response includes the following elements:

* [GetFolderResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getfolderresponse)

* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)

* [GetFolderResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getfolderresponsemessage)

* [Folders](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folders-ex15websvcsotherref)

* [Folder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folder)

* [FolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderid)

* [DisplayName (string)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/displayname-string)

* [TotalCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/totalcount)

* [ChildFolderCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/childfoldercount)

* [UnreadCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/unreadcount)

## GetFolder Error response example

### Description

The following SOAP body example shows an error response that is caused by an incorrect [FolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderid) in the request.

### Code

```xml
<?xml version="1.0" encoding="utf-8" ?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="628" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/en-us/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <GetFolderResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                       xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                       xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:GetFolderResponseMessage ResponseClass="Error">
          <m:MessageText>Id is malformed.</m:MessageText>
          <m:ResponseCode>ErrorInvalidIdMalformed</m:ResponseCode>
          <m:DescriptiveLinkKey>0</m:DescriptiveLinkKey>
          <m:Folders />
        </m:GetFolderResponseMessage>
      </m:ResponseMessages>
    </GetFolderResponse>
  </soap:Body>
</soap:Envelope>
```

### Response elements

This **GetFolder** error response includes the following elements:

* [GetFolderResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getfolderresponse)

* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)

* [GetFolderResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getfolderresponsemessage)

* [MessageText](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/messagetext)

* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)

* [DescriptiveLinkKey](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/descriptivelinkkey)

* [Folders](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folders-ex15websvcsotherref)

## Version differences

For applications that target Exchange Online, Exchange Online as part of Office 365, or an on-premises version of Exchange starting with Exchange 2013, folder permissions are not returned when the [BaseShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape) element has a value of **AllProperties** in the [GetFolder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getfolder-operation) operation request. To retrieve folder permissions, add the [PermissionSet (PermissionSetType)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/permissionset-permissionsettype) element to the [AdditionalProperties](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/additionalproperties) element in the **GetFolder** request.

## See also

* [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)
