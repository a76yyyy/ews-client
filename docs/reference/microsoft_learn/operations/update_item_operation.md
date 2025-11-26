# UpdateItem operation

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem-operation>

The **UpdateItem** operation is used to modify the properties of an existing item in the Exchange store.

You can perform three basic update actions on an item. The following table lists the actions that you can perform.

| Action | Description |
| :--- | :--- |
| Append | Adds data to an existing property. This action preserves the current data. Append does not apply to all properties. |
| Set | Replaces data for a property if the property contains data, or creates the property and sets its value if the property does not exist. The set action is only applicable to writable properties. |
| Delete | Removes a property from an item. This differs from setting a property to an empty value. When this action is finished, the property does not exist for the item. Delete is only applicable to writable properties. |

An **UpdateItem** call can be used to modify one or more items, and one or more properties on each item. The [ItemChanges](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchanges) element contains all the modifications that are to be performed as part of this call. The [ItemChange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchange) element, a child of the [ItemChanges](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchanges) element, represents the modifications to be performed on a single item. The [ItemChange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchange) element contains a set of update actions that can be performed on a single item. These changes are contained in the [Updates (Item)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updates-item) element. The [ItemId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemid) element identifies the item to update. To update more than one property on an item, a [SetItemField](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/setitemfield), [AppendToItemField](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/appendtoitemfield), or [DeleteItemField](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deleteitemfield) must be provided for each property that requires the update.

> [!NOTE]
> Update actions are applied in the order in which they are specified.

For each change, you have to specify the path of the property to change and a representation of that item with its new value. The delete action is slightly different in that only the path of the property that should be deleted is required. For set and append actions, the path that is specified must refer to the same property that is being set in the item representation. If these differ, an error will be returned.

The **UpdateItem** operation can set the [Start](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/start) and [End](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/end-ex15websvcsotherref) time of an Exchange store item. In an **UpdateItem** request, the [Start](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/start) time can be set without also setting the [End](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/end-ex15websvcsotherref) time. This can cause an error if the [Start](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/start) time is later than the [End](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/end-ex15websvcsotherref) time. Be aware that client applications must adjust the [End](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/end-ex15websvcsotherref) time when the [Start](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/start) time is changed in order to preserve the duration.

When a single calendar item is updated to become a recurring master calendar item, the [MeetingTimeZone](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/meetingtimezone) property must be set by the **UpdateItem** operation in order to preserve the calendar item's original time zone.

## SetItemField request example

### Description

The following example of an **UpdateItem** request shows how to set the sensitivity property on an item.

### Code

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema"
               xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <UpdateItem MessageDisposition="SaveOnly" ConflictResolution="AutoResolve"
                xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <ItemChanges>
        <t:ItemChange>
          <t:ItemId Id="AAAtAEFkb..." ChangeKey="CQAAABYAAAB..."/>
          <t:Updates>
            <t:SetItemField>
              <t:FieldURI FieldURI="item:Sensitivity"/>
              <t:Message>
                <t:Sensitivity>Normal</t:Sensitivity>
              </t:Message>
            </t:SetItemField>
          </t:Updates>
        </t:ItemChange>
      </ItemChanges>
    </UpdateItem>
  </soap:Body>
</soap:Envelope>
```

The item identifier and change key have been shortened to preserve readability.

### SetItemField Request Elements

The following elements are used in the request:

* [UpdateItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem)
* [ItemChanges](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchanges)
* [ItemChange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchange)
* [ItemId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemid)
* [Updates (Item)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updates-item)
* [SetItemField](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/setitemfield)
* [FieldURI](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/fielduri)
* [Message](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref)
* [Sensitivity](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/sensitivity)

## AppendToItemField request example

### Description

The following example of an **UpdateItem** request shows how to append text to the body property on an item.

### Code

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema"
  xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
  xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <UpdateItem MessageDisposition="SaveOnly" ConflictResolution="AutoResolve"
                xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <ItemChanges>
        <t:ItemChange>
          <t:ItemId Id="AAAtAEFkbW..." ChangeKey="CQAAABYA..."/>
          <t:Updates>
            <t:AppendToItemField>
              <t:FieldURI FieldURI="item:Body"/>
              <t:Message>
                <t:Body BodyType="Text">Some additional text to append</t:Body>
              </t:Message>
            </t:AppendToItemField>
          </t:Updates>
        </t:ItemChange>
      </ItemChanges>
    </UpdateItem>
  </soap:Body>
</soap:Envelope>
```

The following properties support the append action:

* **message:ReplyTo**
* **item:Body**
* All the recipient and attendee collection properties

The item identifier and change key have been shortened to preserve readability.

### AppendToItemField Request Elements

The following elements are used in the request:

* [UpdateItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem)
* [ItemChanges](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchanges)
* [ItemChange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchange)
* [ItemId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemid)
* [Updates (Item)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updates-item)
* [AppendToItemField](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/appendtoitemfield)
* [FieldURI](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/fielduri)
* [Message](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref)
* [Body](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/body)

## DeleteItemField request example

### Description

The following example of an **UpdateItem** request shows how to delete a property on an item.

### Code

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xmlns:xsd="http://www.w3.org/2001/XMLSchema"
  xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
  <soap:Body>
    <UpdateItem MessageDisposition="SaveOnly" ConflictResolution="AutoResolve"
                xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <ItemChanges>
        <t:ItemChange>
          <t:ItemId Id="AAAtAEFkbWluaXN0cm..." ChangeKey="CQAAABYAA..."/>
          <t:Updates>
            <t:DeleteItemField>
              <t:FieldURI FieldURI="item:Body"/>
            </t:DeleteItemField>
          </t:Updates>
        </t:ItemChange>
      </ItemChanges>
    </UpdateItem>
  </soap:Body>
</soap:Envelope>
```

The item identifier and change key have been shortened to preserve readability.

### DeleteItemField Request Elements

The following elements are used in the request:

* [UpdateItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem)
* [ItemChanges](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchanges)
* [ItemChange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchange)
* [ItemId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemid)
* [Updates (Item)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updates-item)
* [DeleteItemField](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deleteitemfield)
* [FieldURI](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/fielduri)

## Successful response example

### Description

The following example shows a successful response to an **UpdateItem** request.

### Code

```xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
  xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Header>
    <t:ServerVersionInfo MajorVersion="8" MinorVersion="0" MajorBuildNumber="664" MinorBuildNumber="0"
                         xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"/>
  </soap:Header>
  <soap:Body>
    <UpdateItemResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                        xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
      xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
      <m:ResponseMessages>
        <m:UpdateItemResponseMessage ResponseClass="Success">
          <m:ResponseCode>NoError</m:ResponseCode>
          <m:Items>
            <t:Message>
              <t:ItemId Id="AAAtAEFkbW..." ChangeKey="CQAAABYAAA..."/>
            </t:Message>
          </m:Items>
        </m:UpdateItemResponseMessage>
      </m:ResponseMessages>
    </UpdateItemResponse>
  </soap:Body>
</soap:Envelope>
```

The item identifier and change key have been shortened to preserve readability.

### Successful response elements

The following elements are used in the response:

* [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo)
* [UpdateItemResponse](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitemresponse)
* [ResponseMessages](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages)
* [UpdateItemResponseMessage](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitemresponsemessage)
* [ResponseCode](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode)
* [Items](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/items)
* [Message](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref)
* [ItemId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemid)

## See also

[UpdateItem operation (task)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem-operation-task)

[UpdateItem operation (contact)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem-operation-contact)

* [EWS XML elements in Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-xml-elements-in-exchange)

[Updating Contacts](https://msdn.microsoft.com/library/9a865953-b94a-4229-b632-2dee433314be%28Office.15%29.aspx)

[Updating Tasks](https://msdn.microsoft.com/library/0a1bf360-d40c-4a99-929b-4c73a14394d5%28Office.15%29.aspx)
