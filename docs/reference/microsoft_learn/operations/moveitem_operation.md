# MoveItem operation | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/moveitem-operation>

The **MoveItem** operation is used to move one or more items to a single destination folder.

## MoveItem request example

### Description

The following example of a **MoveItem** request shows how to move an item to the Drafts folder.

### Code

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xmlns:xsd="http://www.w3.org/2001/XMLSchema"
xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <MoveItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"
    xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
      <ToFolderId>
        <t:DistinguishedFolderId Id="drafts"/>
      </ToFolderId>
      <ItemIds>
        <t:ItemId Id="AAAtAEF/swbAAA=" ChangeKey="EwAAABYA/s4b"/>
      </ItemIds>
    </MoveItem>
  </soap:Body>
</soap:Envelope>
```

The [ToFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/tofolderid) element specifies the folder to which the items will be moved. Note that all items listed in the [ItemIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemids) collection will end up in the destination folder. You must make separate **MoveItem** calls to place items in different destination folders.

> **Note**
>
> The item identifier and change key have been shortened to preserve readability.

### Request elements

The following elements are used in the request:

* [MoveItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/moveitem)

* [ToFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/tofolderid)

* [DistinguishedFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/distinguishedfolderid)

* [ItemIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemids)

* [ItemId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemid)

## MoveItem response example

### Description

The following example shows a successful response to a **MoveItem** request.

The item identifier of the new item is returned in the response message. Item identifiers are not returned in responses for cross-mailbox or mailbox to public folder **MoveItem** operations.

### Code

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="662" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"/>
  </soap:Header>
  <soap:Body>
    <MoveItemResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                      xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                      xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:MoveItemResponseMessage ResponseClass="Success">
          <m:ResponseCode>NoError</m:ResponseCode>
          <m:Items>
            <t:Message>
              <t:ItemID Id="AAMkAd" ChangeKey="FwAAABY" />
            </t:Message>
          </m:Items>
        </m:MoveItemResponseMessage>
      </m:ResponseMessages>
    </MoveItemResponse>
  </soap:Body>
</soap:Envelope>
```

The **MoveItem** operation will indicate success if the move was successful.

### Successful response elements

The following elements are used in the response:

* [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo)

* [MoveItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/moveitemresponse)

* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)

* [MoveItemResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/moveitemresponsemessage)

* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)

* [Items](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/items)

## See also

* [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)
