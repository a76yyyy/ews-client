# UpdateItem | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem>

The **UpdateItem** element defines a request to update an item in a mailbox.

## XML Structure

```xml
<UpdateItem ConflictResolution="" MessageDisposition="" SendMeetingInvitationsOrCancellations="" SuppressReadReceipts="">
   <SavedItemFolderId/>
   <ItemChanges/>
</UpdateItem>
```

## Attributes and Elements

### Attributes

#### ConflictResolution Attribute

Identifies the type of conflict resolution to try during an update. The default value is AutoResolve.

**Possible Values:**

- **NeverOverwrite** - If there is a conflict, the update operation fails and an error is returned.
- **AutoResolve** - The update operation automatically resolves any conflict.
- **AlwaysOverwrite** - If there is a conflict, the update operation will overwrite information.

#### MessageDisposition Attribute

Describes how the item will be handled after it is updated. The **MessageDisposition** attribute is required for message items, including meeting messages such as meeting cancellations, meeting requests, and meeting responses.

**Possible Values:**

- **SaveOnly** - The item is updated and saved back to its current folder.
- **SendOnly** - The item is updated and sent but no copy is saved.
- **SendAndSaveCopy** - The item is updated and a copy is saved in the folder identified by the [SavedItemFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/saveditemfolderid) element.

#### SendMeetingInvitationsOrCancellations Attribute

Describes how meeting updates are communicated after a calendar item is updated. This attribute is required for calendar items and calendar item occurrences.

**Possible Values:**

- **SendToNone** - The calendar item is updated but updates are not sent to attendees.
- **SendOnlyToAll** - The calendar item is updated and the meeting update is sent to all attendees but is not saved in the Sent Items folder.
- **SendOnlyToChanged** - The calendar item is updated and the meeting update is sent only to attendees that are affected by the change in the meeting.
- **SendToAllAndSaveCopy** - The calendar item is updated, the meeting update is sent to all attendees, and a copy is saved in the Sent Items folder.
- **SendToChangedAndSaveCopy** - The calendar item is updated, the meeting update is sent to all attendees that are affected by the change in the meeting, and a copy is saved in the Sent Items folder.

#### SuppressReadReceipts Attribute

Indicates whether read receipts for the updated item should be suppressed. A text value of **true** indicates that read receipts should be suppressed. A value of **false** indicates that the read receipts will be sent to the sender. This attribute is optional and was introduced in Exchange Server 2013 SP1.

### Child Elements

| **Element** | **Description** |
|-------------|-----------------|
| [SavedItemFolderId](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/saveditemfolderid) | Identifies the target folder for operations that update, send, and create items in the Exchange store. |
| [ItemChanges](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchanges) | Contains an array of [ItemChange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchange) elements that identify items and the updates to apply to the items. |

## Element Information

- **Namespace:** <http://schemas.microsoft.com/exchange/services/2006/messages>
- **Schema Name:** Messages schema
- **Validation File:** Messages.xsd
- **Can be Empty:** False

## Remarks

The schema that describes this element is located in the IIS virtual directory that hosts Exchange Web Services.

## See Also

- [UpdateItem operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem-operation)
