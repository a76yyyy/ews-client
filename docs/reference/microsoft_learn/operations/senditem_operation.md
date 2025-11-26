# SendItem operation | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/senditem-operation>

The SendItem operation is used to send e-mail messages that are located in the Exchange store.

## SendItem (E-mail Message) request example

### Description

The following example shows how to send an e-mail message.

### Code

```xml
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema"
               xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <SendItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"
              SaveItemToFolder="true">
      <ItemIds>
        <t:ItemId Id="AAAtAEF=" ChangeKey="CQAAABY+T" />
      </ItemIds>
    </SendItem>
  </soap:Body>
</soap:Envelope>
```

The item identifier has been shortened to preserve readability.

### Request elements

The following elements are used in the request:

* [SendItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/senditem)

* [ItemIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemids)

* [ItemId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemid)

## Successful SendItem (E-mail Message) Response

### Description

The following example shows a successful SendItem response.

### Code

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="602" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/en-us/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <SendItemResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                      xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                      xmlns="http://schemas.microsoft.com/enchange/services/2006/messages">
      <m:ResponseMessages>
        <m:SendItemResponseMessage ResponseClass="Success">
          <m:ResponseCode>NoError</m:ResponseCode>
        </m:SendItemResponseMessage>
      </m:ResponseMessages>
    </SendItemResponse>
  </soap:Body>
</soap:Envelope>
```

### Successful response elements

The following elements are used in the response:

* [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo)

* [SendItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/senditemresponse)

* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)

* [SendItemResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/senditemresponsemessage)

* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)

A delegate who tries to send an e-mail message that is located in the principal's Drafts folder with the SendAndSaveCopy option set to save a copy in the Sent Items distinguished folder will silently fail to move a copy of the sent item to the Sent Items distinguished folder. The item will remain in the principal's Drafts folder. The workaround for this issue is to specify the principal's mailbox in the [DistinguishedFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/distinguishedfolderid) element.

An additional scenario to consider is when a delegate creates an e-mail message and saves it to the Drafts folder of the delegate's mailbox. If the delegate tries to send the item and save a copy to the principal's Sent Items distinguished folder, the message is sent correctly, the draft message remains in the delegate's Drafts folder, the sent message does not appear in either the delegate's or principal's Sent Items folder, and the response is a success.

## Invalid SendItem (E-mail Message) request example

### Description

The following code sample shows an example of a request with an invalid identifier.

### Code

```xml
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema"
               xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <SendItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"
              SaveItemToFolder="true">
      <ItemIds>
        <t:ItemId Id="%BadItemId%" ChangeKey="CQAAABYAAA" />
      </ItemIds>
    </SendItem>
  </soap:Body>
</soap:Envelope>
```

## SendItem (E-mail Message) error response

### Description

The following example shows an error response to a SendItem request that contains an invalid identifier.

### Code

```xml
<?xml version="1.0" encoding="utf-8" ?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="602" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <SendItemResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                      xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                      xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:SendItemResponseMessage ResponseClass="Error">
          <m:MessageText>Id is malformed.</m:MessageText>
          <m:ResponseCode>ErrorInvalidIdMalformed</m:ResponseCode>
          <m:DescriptiveLinkKey>0</m:DescriptiveLinkKey>
        </m:SendItemResponseMessage>
      </m:ResponseMessages>
    </SendItemResponse>
  </soap:Body>
</soap:Envelope>
```

### Error response elements

The following elements are used in the error response:

* [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo)

* [SendItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/senditemresponse)

* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)

* [SendItemResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/senditemresponsemessage)

* [MessageText](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/messagetext)

* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)

* [DescriptiveLinkKey](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/descriptivelinkkey)

## See also

[SendItem operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/senditem-operation)

**SendItemType**

* [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)
