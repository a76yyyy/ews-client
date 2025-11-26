# DeleteItem operation | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deleteitem-operation>

The **DeleteItem** operation deletes items in the Exchange store.

> **Note**
>
> An error response that includes the ErrorCannotDeleteObject error code will be returned for a **DeleteItem** operation when a delegate tries to delete an item in the principal's mailbox by setting the DisposalType to MoveToDeletedItems. To delete an item by moving it to the Deleted Items folder, a delegate must use the [MoveItem operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/moveitem-operation).

## DeleteItem request example

### Description

The following example of a **DeleteItem** request shows how to perform a hard delete of an item from a mailbox.

> **Note**
>
> The item ID has been shortened to preserve readability.

### Code

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
  xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <DeleteItem DeleteType="HardDelete" xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <ItemIds>
        <t:ItemId Id="AS4AUn=="/>
      </ItemIds>
    </DeleteItem>
  </soap:Body>
</soap:Envelope>
```

### Request elements

The following elements are used in the request:

* [DeleteItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deleteitem)

* [ItemIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemids)

* [ItemId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemid)

To find other options for the request message of the **DeleteItem** operation, explore the schema hierarchy. Start at the [DeleteItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deleteitem) element.

## Successful DeleteItem response

### Description

The following example shows a successful response to the **DeleteItem** request.

### Code

```xml
<?xml version="1.0" encoding="utf-8" ?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="595" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/en-us/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <DeleteItemResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                        xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                        xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:DeleteItemResponseMessage ResponseClass="Success">
          <m:ResponseCode>NoError</m:ResponseCode>
        </m:DeleteItemResponseMessage>
      </m:ResponseMessages>
    </DeleteItemResponse>
  </soap:Body>
</soap:Envelope>
```

### Successful response elements

The following elements are used in the response:

* [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo)

* [DeleteItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deleteitemresponse)

* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)

* [DeleteItemResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deleteitemresponsemessage)

* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)

To find other options for the response message of the **DeleteItem** operation, explore the schema hierarchy. Start at the [DeleteItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deleteitemresponse) element.

## DeleteItem error response

### Description

The following example shows an error response to a **DeleteItem** request. The error was created because the operation tried to delete an item that was not found in the Exchange store.

### Code

```xml
<?xml version="1.0" encoding="utf-8" ?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="595" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/en-us/exchange/services/2006/types" />
  </soap:Header>
<soap:Body>
    <DeleteItemResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                        xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                        xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:DeleteItemResponseMessage ResponseClass="Error">
          <m:MessageText>The specified object was not found in the store.</m:MessageText>
          <m:ResponseCode>ErrorItemNotFound</m:ResponseCode>
          <m:DescriptiveLinkKey>0</m:DescriptiveLinkKey>
        </m:DeleteItemResponseMessage>
      </m:ResponseMessages>
    </DeleteItemResponse>
  </soap:Body>
</soap:Envelope>
```

### Error response elements

The following elements are used in the error response:

* [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo)

* [DeleteItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deleteitemresponse)

* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)

* [DeleteItemResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deleteitemresponsemessage)

* [MessageText](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/messagetext)

* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)

* [DescriptiveLinkKey](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/descriptivelinkkey)

To find other options for the error response message of the **DeleteItem** operation, explore the schema hierarchy. Start at the [DeleteItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deleteitemresponse) element.

## See also

* [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)
* [Deleting Contacts](https://msdn.microsoft.com/library/fcc3dc84-cd3e-455e-a1a7-ae6921c9b588%28Office.15%29.aspx)
* [Deleting E-mail Messages](https://msdn.microsoft.com/library/c40f2f0b-dae0-412f-b716-727e8c0949b4%28Office.15%29.aspx)
* [Deleting Tasks](https://msdn.microsoft.com/library/a3d7e25f-8a35-4901-b1d9-d31f418ab340%28Office.15%29.aspx)
