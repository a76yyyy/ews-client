# SyncFolderHierarchy operation

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderhierarchy-operation>

The SyncFolderHierarchy operation synchronizes folders between the computer that is running Microsoft Exchange Server 2010 and the client.

> [!NOTE]
> The SyncFolderHierarchy operation does not return folders when the [UnreadCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/unreadcount) or [TotalCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/totalcount) properties have changed.

## SyncFolderHierarchy request example

### Description

The following example of a SyncFolderHierarchy request shows how to synchronize a client folder hierarchy with the Exchange server. This example shows a folder hierarchy that has already been synchronized at least one time. The [SyncState](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncstate-ex15websvcsotherref) element is not included in the request for the first attempt to synchronize a client with the Exchange server. The first request will return all the folders in the mailbox. The [SyncState](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncstate-ex15websvcsotherref) element will be returned in the [SyncFolderHierarchyResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderhierarchyresponse). This element is used to synchronize the state for subsequent SyncFolderHierarchy requests.

### Code

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
  xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <SyncFolderHierarchy  xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <FolderShape>
        <t:BaseShape>AllProperties</t:BaseShape>
      </FolderShape>
      <SyncState>H4sIA=</SyncState>
    </SyncFolderHierarchy>
  </soap:Body>
</soap:Envelope>
```

The [SyncState](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncstate-ex15websvcsotherref) element base64-encoded data has been shortened to preserve readability.

### Request elements

The following elements are used in the request:

* [SyncFolderHierarchy](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderhierarchy)
* [FolderShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/foldershape)
* [BaseShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape)
* [SyncState](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncstate-ex15websvcsotherref)

> [!NOTE]
> The schema that describes these elements is located in the EWS virtual directory of the computer that is running MicrosoftExchange Server 2007 that has the Client Access server role installed.

## Successful SyncFolderHierarchy Response

### Description

The following example shows a successful response to the SyncFolderHierarchy request. In this example, a new folder has been synchronized.

### Code

```xml
<?xml version="1.0" encoding="utf-8" ?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0"
                         MajorBuildNumber="628" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <SyncFolderHierarchyResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                                 xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                                 xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:SyncFolderHierarchyResponseMessage ResponseClass="Success">
          <m:ResponseCode>NoError</m:ResponseCode>
          <m:SyncState>H4sIAAA==</m:SyncState>
          <m:IncludesLastFolderInRange>true</m:IncludesLastFolderInRange>
          <m:Changes>
            <t:Create>
              <t:Folder>
                <t:FolderId Id="AQApAHR=" ChangeKey="AQAAABY" />
                <t:ParentFolderId Id="AQApA=" ChangeKey="AQAAAA==" />
                <t:FolderClass>IPF.Note</t:FolderClass>
                <t:DisplayName>NewFolder</t:DisplayName>
                <t:TotalCount>0</t:TotalCount>
                <t:ChildFolderCount>0</t:ChildFolderCount>
                <t:UnreadCount>0</t:UnreadCount>
              </t:Folder>
            </t:Create>
          </m:Changes>
        </m:SyncFolderHierarchyResponseMessage>
      </m:ResponseMessages>
    </SyncFolderHierarchyResponse>
  </soap:Body>
</soap:Envelope>
```

The [SyncState](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncstate-ex15websvcsotherref) element base64-encoded data and the folder identifier data have been shortened to preserve readability.

### Successful response elements

The following elements are used in the response:

* [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo)
* [SyncFolderHierarchyResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderhierarchyresponse)
* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)
* [SyncFolderHierarchyResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderhierarchyresponsemessage)
* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)
* [SyncState](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncstate-ex15websvcsotherref)
* [IncludesLastFolderInRange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/includeslastfolderinrange)
* [Changes (Hierarchy)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/changes-hierarchy)
* [Create (FolderSync)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/create-foldersync)
* [Folder](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folder)
* [FolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderid)
* [ParentFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/parentfolderid)
* [FolderClass](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderclass)
* [DisplayName (string)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/displayname-string)
* [TotalCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/totalcount)
* [ChildFolderCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/childfoldercount)
* [UnreadCount](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/unreadcount)

## SyncFolderHierarchy error response

### Description

The following example shows an error response to a SyncFolderHierarchy request. This error was caused by an invalid SyncState.

### Code

```xml
<?xml version="1.0" encoding="utf-8" ?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0"
                         MajorBuildNumber="628" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <SyncFolderHierarchyResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                                 xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                                 xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:SyncFolderHierarchyResponseMessage ResponseClass="Error">
          <m:MessageText>Synchronization state data is corrupted or otherwise invalid.</m:MessageText>
          <m:ResponseCode>ErrorInvalidSyncStateData</m:ResponseCode>
          <m:DescriptiveLinkKey>0</m:DescriptiveLinkKey>
          <m:SyncState />
          <m:IncludesLastFolderInRange>true</m:IncludesLastFolderInRange>
        </m:SyncFolderHierarchyResponseMessage>
      </m:ResponseMessages>
    </SyncFolderHierarchyResponse>
  </soap:Body>
</soap:Envelope>
```

### Error response elements

The following elements are used in the error response:

* [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo)
* [SyncFolderHierarchyResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderhierarchyresponse)
* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)
* [SyncFolderHierarchyResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncfolderhierarchyresponsemessage)
* [MessageText](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/messagetext)
* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)
* [DescriptiveLinkKey](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/descriptivelinkkey)
* [SyncState](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/syncstate-ex15websvcsotherref)
* [IncludesLastFolderInRange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/includeslastfolderinrange)

## See also

* [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)
