# MarkAllItemsAsRead operation

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/markallitemsasread-operation>

Find information about the **MarkAllItemsAsRead** EWS operation.

The **MarkAllItemsAsRead** operation sets the [IsRead](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/isread) property on all items, in one or more folders, to indicate that all items are either read or unread.

This operation was introduced in Exchange Server 2013.

## Using the MarkAllItemsAsRead operation

The **MarkAllItemsAsRead** operation can set the [IsRead](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/isread) property on all items in folders identified by either the Exchange Web Services (EWS) folder identifier or the default Exchange folder name. The **MarkAllItemsAsRead** operation can also suppress the sending of read receipts for items marked as read.

The **MarkAllItemsAsRead** operation can use the SOAP headers that are listed in the following table.

| Header name | Element | Description |
| :--- | :--- | :--- |
| **Impersonation** | [ExchangeImpersonation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/exchangeimpersonation) | Identifies the user whom the client application is impersonating. This header is applicable to a request. |
| **MailboxCulture** | [MailboxCulture](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailboxculture) | Identifies the culture, as defined in RFC 3066, "Tags for the Identification of Languages", to be used to access the mailbox. This header is applicable to a request. |
| **RequestVersion** | [RequestServerVersion](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/requestserverversion) | Identifies the schema version for the operation request. This header is applicable to a request. |
| **ServerVersion** | [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo) | Identifies the version of the server that responded to the request. This header is applicable to a response. |

## MarkAllItemsAsRead operation request example: Mark all items in a folder as read

The following example of a **MarkAllItemsAsRead** operation request shows how to set the [IsRead](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/isread) property, which is also called the read flag, to **true** on all items in a folder. This example also shows that read receipts are not sent in response to any read receipt requests.

> [!NOTE]
> All item identifiers and change keys in this article have been shortened to preserve readability. Change keys are not required for this operation.

```xml
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
               xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
               xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
   <soap:Header>
      <t:RequestServerVersion Version="Exchange2013" />
   </soap:Header>
   <soap:Body>
      <m:MarkAllItemsAsRead>
         <m:ReadFlag>true</m:ReadFlag>
         <m:SuppressReadReceipts>true</m:SuppressReadReceipts>
         <m:FolderIds>
            <t:FolderId Id="AAMkADEzOTExYZRAAA="
                        ChangeKey="AQAAAAA3vA==" />
         </m:FolderIds>
      </m:MarkAllItemsAsRead>
   </soap:Body>
</soap:Envelope>
```

The request SOAP body contains the following elements:

* [MarkAllItemsAsRead](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/markallitemsasread)
* [ReadFlag](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/readflag)
* [SuppressReadReceipts](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/suppressreadreceipts)
* [FolderIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderids)
* [FolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderid)

## Successful MarkAllItemsAsRead operation response

The following example shows a successful response to a **MarkAllItemsAsRead** operation request to mark all items in a folder as read.

```xml
<?xml version="1.0" encoding="utf-8"?>
<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
   <s:Header>
      <h:ServerVersionInfo MajorVersion="15"
                           MinorVersion="0"
                           MajorBuildNumber="545"
                           MinorBuildNumber="11"
                           Version="Exchange2013"
                           xmlns:h="http://schemas.microsoft.com/exchange/services/2006/types"
                           xmlns="http://schemas.microsoft.com/exchange/services/2006/types"
                           xmlns:xsd="http://www.w3.org/2001/XMLSchema"
                           xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" />
   </s:Header>
   <s:Body xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
           xmlns:xsd="http://www.w3.org/2001/XMLSchema">
      <m:MarkAllItemsAsReadResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                                    xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
         <m:ResponseMessages>
            <m:MarkAllItemsAsReadResponseMessage ResponseClass="Success">
               <m:ResponseCode>NoError</m:ResponseCode>
            </m:MarkAllItemsAsReadResponseMessage>
         </m:ResponseMessages>
      </m:MarkAllItemsAsReadResponse>
   </s:Body>
</s:Envelope>
```

The response SOAP body contains the following elements:

* [MarkAllItemsAsReadResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/markallitemsasreadresponse)
* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)
* [MarkAllItemsAsReadResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/markallitemsasreadresponsemessage)
* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)

## MarkAllItemsAsRead operation request example: Mark all items in a folder as unread

The following example of a **MarkAllItemsAsRead** operation request shows how to set the [IsRead](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/isread) property to **false** on all items in a folder. This example also shows that read receipts are not sent in response to any read receipt requests.

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
               xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
               xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
   <soap:Header>
      <t:RequestServerVersion Version="Exchange2013" />
   </soap:Header>
   <soap:Body>
      <m:MarkAllItemsAsRead>
         <m:ReadFlag>false</m:ReadFlag>
         <m:SuppressReadReceipts>true</m:SuppressReadReceipts>
         <m:FolderIds>
            <t:FolderId Id="AAMkADEzOTExYZRAAA="
                        ChangeKey="AQAAAAA3vA==" />
         </m:FolderIds>
      </m:MarkAllItemsAsRead>
   </soap:Body>
</soap:Envelope>
```

A successful response to a request to mark all items as read is the same as the response to a request to mark all items as unread.

The request SOAP body contains the following elements:

* [MarkAllItemsAsRead](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/markallitemsasread)
* [ReadFlag](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/readflag)
* [SuppressReadReceipts](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/suppressreadreceipts)
* [FolderIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderids)
* [FolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderid)

## MarkAllItemsAsRead operation error response

The following example shows an error response to a **MarkAllItemsAsRead** operation request to mark all items in a folder as read or unread when the folder does not exist in the mailbox.

```xml
<?xml version="1.0" encoding="utf-8"?>
<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
   <s:Header>
      <h:ServerVersionInfo MajorVersion="15"
                           MinorVersion="0"
                           MajorBuildNumber="545"
                           MinorBuildNumber="11"
                           Version="Exchange2013"
                           xmlns:h="http://schemas.microsoft.com/exchange/services/2006/types"
                           xmlns="http://schemas.microsoft.com/exchange/services/2006/types"
                           xmlns:xsd="http://www.w3.org/2001/XMLSchema"
                           xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>
   </s:Header>
   <s:Body xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
           xmlns:xsd="http://www.w3.org/2001/XMLSchema">
      <m:MarkAllItemsAsReadResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                                    xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
         <m:ResponseMessages>
            <m:MarkAllItemsAsReadResponseMessage ResponseClass="Error">
               <m:MessageText>The specified object was not found in the store.</m:MessageText>
               <m:ResponseCode>ErrorItemNotFound</m:ResponseCode>
               <m:DescriptiveLinkKey>0</m:DescriptiveLinkKey>
            </m:MarkAllItemsAsReadResponseMessage>
         </m:ResponseMessages>
      </m:MarkAllItemsAsReadResponse>
   </s:Body>
</s:Envelope>
```

The error response SOAP body contains the following elements:

* [MarkAllItemsAsReadResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/markallitemsasreadresponse)
* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)
* [MarkAllItemsAsReadResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/markallitemsasreadresponsemessage)
* [MessageText](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/messagetext)
* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)
* [DescriptiveLinkKey](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/descriptivelinkkey)

## See also

* [EWS operations in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-operations-in-exchange)
* [FindFolder operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/findfolder-operation)
