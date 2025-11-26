# CopyItem operation | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/copyitem-operation>

The **CopyItem** operation copies items and puts the items in a different folder.

## CopyItem request example

### Description

The following example of a **CopyItem** request shows how to form a request to copy an item to the Inbox.

### Code

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
  xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <CopyItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <ToFolderId>
        <t:DistinguishedFolderId Id="inbox"/>
      </ToFolderId>
      <ItemIds>
        <t:ItemId Id="AS4AUnV="/>
      </ItemIds>
    </CopyItem>
  </soap:Body>
</soap:Envelope>
```

> **Note**
>
> The folder ID and the change key have been shortened to preserve readability.

### Request elements

The following elements are used in the request:

* [CopyItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/copyitem)

* [ToFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/tofolderid)

* [DistinguishedFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/distinguishedfolderid)

* [ItemIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemids)

* [ItemId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemid)

> **Note**
>
> The schema that describes this element is located in the EWS virtual directory of the computer that is running Microsoft Exchange Server 2010 that has the Client Access server role installed.

To find other options for the request message of the **CopyItem** operation, explore the schema hierarchy. Start at the [CopyItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/copyitem) element.

## Successful CopyItem Response

### Description

The following example shows a successful response to the **CopyItem** request.

The item identifier of the new item is returned in the response message. Item identifiers are not returned in responses for cross-mailbox or mailbox to public folder **CopyItem** operations.

### Code

```xml
<?xml version="1.0" encoding="utf-8" ?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="595" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <CopyItemResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                      xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                      xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:CopyItemResponseMessage ResponseClass="Success">
          <m:ResponseCode>NoError</m:ResponseCode>
          <m:Items>
            <t:Message>
              <t:ItemID Id="AAMkAd" ChangeKey="FwAAABY" />
            </t:Message>
          </m:Items>
        </m:CopyItemResponseMessage>
      </m:ResponseMessages>
    </CopyItemResponse>
  </soap:Body>
</soap:Envelope>
```

### Successful response elements

The following elements are used in the response:

* [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo)

* [CopyItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/copyitemresponse)

* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)

* [CopyItemResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/copyitemresponsemessage)

* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)

* [Items](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/items)

To find other options for the response message of the **CopyItem** operation, explore the schema hierarchy. Start at the [CopyItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/copyitemresponse) element.

## CopyItem error response

### Description

The following example shows an error response to a **CopyItem** request.

### Code

```xml
<?xml version="1.0" encoding="utf-8" ?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="595" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <CopyItemResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                      xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                      xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:CopyItemResponseMessage ResponseClass="Error">
          <m:MessageText>Id is malformed.</m:MessageText>
          <m:ResponseCode>ErrorInvalidIdMalformed</m:ResponseCode>
          <m:DescriptiveLinkKey>0</m:DescriptiveLinkKey>
          <m:Items />
        </m:CopyItemResponseMessage>
      </m:ResponseMessages>
    </CopyItemResponse>
  </soap:Body>
</soap:Envelope>
```

### Error response elements

The following elements are used in the error response:

* [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo)

* [CopyItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/copyitemresponse)

* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)

* [CopyItemResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/copyitemresponsemessage)

* [MessageText](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/messagetext)

* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)

* [DescriptiveLinkKey](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/descriptivelinkkey)

* [Items](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/items)

To find other options for the error response message of the **CopyItem** operation, explore the schema hierarchy. Start at the [CopyItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/copyitemresponse) element.

## See also

* [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)
