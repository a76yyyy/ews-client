# FindItem operation | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation>

Find information about the **FindItem** EWS operation.

The **FindItem** operation searches for items that are located in a user's mailbox. This operation provides many ways to filter and format how search results are returned to the caller.

## Using the FindItem operation

The **FindItem** operation request provides many ways for you to search a mailbox and format how the data is returned in a response. You can specify the following in a **FindItem** request:

* Whether the search is a shallow or soft-deleted traversal. Specifying this is required. Note that a soft-deleted traversal combined with a search restriction will result in zero items returned, even if there are items that match the search criteria.

* The response shape of items. This identifies the properties that are returned in the response. Specifying this is required.

* The folders from which to perform the search. Specifying this is required.

* The paging mechanism and view types for returning view data in pages. Specifying this is optional.

* Options for grouping and sorting the items that are returned. Specifying this is optional.

* Search restrictions or Advanced Query Syntax (AQS) strings for filtering the items that are returned. For more information about using AQS for content index searches, see [QueryString (String)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/querystring-string). Specifying this is optional.

* The sort order for items returned in the response. Specifying this is optional.

The **FindItem** operation returns only the first 512 bytes of any streamable property. For Unicode, it returns the first 255 characters by using a null-terminated Unicode string. It does not return any of the message body formats or the recipient lists. **FindItem** will return a recipient summary. You can use the [GetItem operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getitem-operation) to get the details of an item.

**FindItem** returns only the [Name (EmailAddressType)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/name-emailaddresstype) element and does not return the [EmailAddress (NonEmptyStringType)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/emailaddress-nonemptystringtype) element in the [Mailbox](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailbox) element for the following fields:

* The [From](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/from) field for messages

* The [Sender](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/sender) field for messages

* The [Organizer](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/organizer) field for calendar items

> **Note**
>
> The **FindItem** operation can return results in a [CalendarView](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/calendarview) element. The **CalendarView** element returns single calendar items and all occurrences of recurring meetings. If a **CalendarView** element is not used, single calendar items and recurring master calendar items are returned. The occurrences must be expanded from the recurring master if a **CalendarView** element is not used.

The **FindItem** operation can use the SOAP headers that are listed in the following table.

**Table 1. FindItem operation SOAP headers**

| **Header** | **Element** | **Description** |
|------------|-------------|-----------------|
| **DateTimePrecision** | [DateTimePrecision](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/datetimeprecision) | Specifies the resolution of data/time values in responses from the server, either in seconds or in milliseconds. This is applicable to a request. |
| **Impersonation** | [ExchangeImpersonation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/exchangeimpersonation) | Identifies the user whom the client application is impersonating. This is applicable to a request. |
| **MailboxCulture** | [MailboxCulture](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailboxculture) | Identifies the RFC3066 culture to be used to access the mailbox. This is applicable to a request. |
| **RequestVersion** | [RequestServerVersion](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/requestserverversion) | Identifies the schema version for the operation request. This is applicable to a request. |
| **ServerVersion** | [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo) | Identifies the version of the server that responded to the request. This is applicable to a response. |
| **TimeZoneContext** | [TimeZoneContext](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/timezonecontext) | Identifies the time zone to be used for all responses from the server. This is applicable to a request. |

## FindItem operation request example

The following example of a **FindItem** request shows how to obtain the item identifier that is defined by the **IdOnly** enumeration of the [BaseShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape) element for items that are found in the Deleted Items folder.

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <FindItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"
               xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
              Traversal="Shallow">
      <ItemShape>
        <t:BaseShape>IdOnly</t:BaseShape>
      </ItemShape>
      <ParentFolderIds>
        <t:DistinguishedFolderId Id="deleteditems"/>
      </ParentFolderIds>
    </FindItem>
  </soap:Body>
</soap:Envelope>
```

The following elements are used in the request:

* [FindItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem)

* [ItemShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemshape)

* [BaseShape](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape)

* [ParentFolderIds](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/parentfolderids)

* [DistinguishedFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/distinguishedfolderid)

For more options for a **FindItem** request message, explore the schema hierarchy. Start at the [FindItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem) element.

## Successful FindItem operation response

The following example shows a successful response to the **FindItem** request.

[Message](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref) elements represent email messages and all other items that are not strongly typed by the EWS schema. Items such as IPM.Sharing and IPM.InfoPath are returned as [Message](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref) elements. Exchange does not return the base [Item](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/item) element in responses.

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="595" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types" />
  </soap:Header>
  <soap:Body>
    <FindItemResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                      xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                      xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:FindItemResponseMessage ResponseClass="Success">
          <m:ResponseCode>NoError</m:ResponseCode>
          <m:RootFolder TotalItemsInView="10" IncludesLastItemInRange="true">
            <t:Items>
              <t:Message>
                <t:ItemId Id="AS4AUn=" ChangeKey="fsVU4==" />
              </t:Message>
              <t:Message>
                <t:ItemId Id="AS4AUM=" ChangeKey="fsVUA==" />
              </t:Message>
            </t:Items>
          </m:RootFolder>
        </m:FindItemResponseMessage>
      </m:ResponseMessages>
    </FindItemResponse>
  </soap:Body>
</soap:Envelope>
```

The following elements are used in the response:

* [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo)

* [FindItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditemresponse)

* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)

* [FindItemResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditemresponsemessage)

* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)

* [RootFolder (FindItemResponseMessage)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/rootfolder-finditemresponsemessage)

* [Items](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/items)

* [Message](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref)

* [ItemId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemid)
