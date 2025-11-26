# CreateItem operation (email message)

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitem-operation-email-message>

The CreateItem operation is used to create e-mail messages.

## CreateItem request example

### Description

The following example of a CreateItem request shows how to create a new e-mail message, send the message, and save a copy of it in the drafts folder.

### Code

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
  xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <CreateItem MessageDisposition="SendAndSaveCopy" xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <SavedItemFolderId>
        <t:DistinguishedFolderId Id="drafts" />
      </SavedItemFolderId>
      <Items>
        <t:Message>
          <t:ItemClass>IPM.Note</t:ItemClass>
          <t:Subject>Project Action</t:Subject>
          <t:Body BodyType="Text">Priority - Update specification</t:Body>
          <t:ToRecipients>
            <t:Mailbox>
              <t:EmailAddress>sschmidt@example.com</t:EmailAddress>
            </t:Mailbox>
          </t:ToRecipients>
          <t:IsRead>false</t:IsRead>
        </t:Message>
      </Items>
    </CreateItem>
  </soap:Body>
</soap:Envelope>
```

### Request elements

The following elements are used in the request:

* [CreateItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitem)
* [SavedItemFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/saveditemfolderid)
* [Items (NonEmptyArrayOfAllItemsType)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/items-nonemptyarrayofallitemstype)
* [Message](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref)
* [ItemClass](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemclass)
* [Subject](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/subject)
* [Body](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/body)
* [ToRecipients](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/torecipients)
* [Mailbox](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailbox)
* [EmailAddress (NonEmptyStringType)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/emailaddress-nonemptystringtype)
* [IsRead](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/isread)

To find other options for the request message of the CreateItem operation, explore the schema hierarchy. Start at the [CreateItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitem) element.

## Successful CreateItem Response

### Description

The following example shows a successful response to the CreateItem request.

### Code

```xml
<?xml version="1.0" encoding="utf-8" ?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="595" MinorBuildNumber="0" xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <CreateItemResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                        xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                        xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:CreateItemResponseMessage ResponseClass="Success">
          <m:ResponseCode>NoError</m:ResponseCode>
          <m:Items />
        </m:CreateItemResponseMessage>
      </m:ResponseMessages>
    </CreateItemResponse>
  </soap:Body>
</soap:Envelope>
```

### Successful response elements

The following elements are included in the response:

* [CreateItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitemresponse)
* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)
* [CreateItemResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitemresponsemessage)
* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)
* [Items](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/items)

To find other options for the response message of the CreateItem operation, explore the schema hierarchy. Start at the [CreateItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitemresponse) element.

## Error CreateItem Response

### Description

The following example shows an error response to a CreateItem request.

### Code

```xml
<?xml version="1.0" encoding="utf-8" ?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="595" MinorBuildNumber="0" xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <CreateItemResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                        xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                        xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:CreateItemResponseMessage ResponseClass="Error">
          <m:MessageText>The user account which was used to submit this request does not have the right to send mail on behalf of the specified sending account.</m:MessageText>
          <m:ResponseCode>ErrorSendAsDenied</m:ResponseCode>
          <m:DescriptiveLinkKey>0</m:DescriptiveLinkKey>
          <m:Items />
        </m:CreateItemResponseMessage>
      </m:ResponseMessages>
    </CreateItemResponse>
  </soap:Body>
</soap:Envelope>
```

### Error response elements

The following elements are used in the error response:

* [CreateItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitemresponse)
* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)
* [CreateItemResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitemresponsemessage)
* [MessageText](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/messagetext)
* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)
* [DescriptiveLinkKey](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/descriptivelinkkey)
* [Items](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/items)

To find other options for the error response message of the CreateItem operation, explore the schema hierarchy. Start at the [CreateItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitemresponse) element.

## See also

[CreateItem operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitem-operation)
