# FieldURI | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/fielduri>

The **FieldURI** element identifies specific properties in Exchange Web Services (EWS) operations.

## Folder Properties

### Basic Folder Properties

- **folder:FolderId** - Identifies the **FolderId** property.
- **folder:ParentFolderId** - Identifies the **ParentFolderId** property.
- **folder:DisplayName** - Identifies the **DisplayName** property.
- **folder:UnreadCount** - Identifies the **UnreadCount** property.
- **folder:TotalCount** - Identifies the **TotalCount** property.
- **folder:ChildFolderCount** - Identifies the **ChildFolderCount** property.
- **folder:FolderClass** - Identifies the **FolderClass** property.
- **folder:SearchParameters** - Identifies the **SearchParameters** property.
- **folder:ManagedFolderInformation** - Identifies the **ManagedFolderInformation** property.
- **folder:PermissionSet** - Identifies the **PermissionSet** property.
- **folder:EffectiveRights** - Identifies the **EffectiveRights** property.
- **folder:SharingEffectiveRights** - Identifies the **SharingEffectiveRights** property.

## Item Properties

### Basic Item Properties

- **item:ItemId** - Identifies the **ItemId** property.
- **item:ParentFolderId** - Identifies the **ParentFolderId** property.
- **item:ItemClass** - Identifies the **ItemClass** property.
- **item:MimeContent** - Identifies the **MimeContent** property.
- **item:Attachments** - Identifies the **Attachments** property.
- **item:Subject** - Identifies the **Subject** property.
- **item:DateTimeReceived** - Identifies the **DateTimeReceived** property.
- **item:Size** - Identifies the **Size** property.
- **item:Categories** - Identifies the **Categories** property.
- **item:HasAttachments** - Identifies the **HasAttachments** property.
- **item:Importance** - Identifies the **Importance** property.
- **item:InReplyTo** - Identifies the **InReplyTo** property.
- **item:InternetMessageHeaders** - Identifies the **InternetMessageHeaders** property.
- **item:IsAssociated** - Identifies the **IsAssociated** property.
- **item:IsDraft** - Identifies the **IsDraft** property.
- **item:IsFromMe** - Identifies the **IsFromMe** property.
- **item:IsResend** - Identifies the **IsResend** property.
- **item:IsSubmitted** - Identifies the **IsSubmitted** property.
- **item:IsUnmodified** - Identifies the **IsUnmodified** property.
- **item:DateTimeSent** - Identifies the **DateTimeSent** property.
- **item:DateTimeCreated** - Identifies the **DateTimeCreated** property.

### Body and Content Properties

- **item:Body** - Identifies the **Body** property.
- **item:UniqueBody** - Identifies the **UniqueBody** property.
- **item:NormalizedBody** - Identifies the **NormalizedBody** property.
- **item:TextBody** - Identifies the **TextBody** property.
- **item:MimeContentUTF8** - Identifies the **MimeContentUTF8** property.

### Response and Reminder Properties

- **item:ResponseObjects** - Identifies the **ResponseObjects** property.
- **item:Sensitivity** - Identifies the **Sensitivity** property.
- **item:ReminderDueBy** - Identifies the **ReminderDueBy** property.
- **item:ReminderIsSet** - Identifies the **ReminderIsSet** property.
- **item:ReminderNextTime** - Identifies the **ReminderNextTime** property.
- **item:ReminderMinutesBeforeStart** - Identifies the **ReminderMinutesBeforeStart** property.

### Display Properties

- **item:DisplayTo** - Identifies the **DisplayTo** property.
- **item:DisplayCc** - Identifies the **DisplayCc** property.

### Metadata Properties

- **item:Culture** - Identifies the **Culture** property.
- **item:EffectiveRights** - Identifies the **EffectiveRights** property.
- **item:LastModifiedName** - Identifies the **LastModifiedName** property.
- **item:LastModifiedTime** - Identifies the **LastModifiedTime** property.
- **item:ConversationId** - Identifies the **ConversationId** property.

### Flag and Status Properties

- **item:Flag** - Identifies the **Flag** property.
- **item:StoreEntryId** - Identifies the **StoreEntryId** property.
- **item:InstanceKey** - Identifies the **InstanceKey** property.

### Advanced Properties

- **item:EntityExtractionResult** - Identifies the **EntityExtractionResult** property.
- **itemPolicyTag** - Identifies the **PolicyTag** property.
- **item:ArchiveTag** - Identifies the **ArchiveTag** property.
- **item:RetentionDate** - Identifies the **RetentionDate** property.
- **item:Preview** - Identifies the **Preview** property.
- **item:NextPredictedAction** - Identifies the **NextPredictedAction** property.
- **item:GroupingAction** - Identifies the **GroupingAction** property.
- **item:PredictedActionReasons** - Identifies the **PredictedActionReasons** property.
- **item:RightsManagementLicenseData** - Identifies the **RightsManagementLicenseData** property.
- **item:BlockStatus** - Identifies the **BlockStatus** property.
- **item:HasBlockedImages** - Identifies the **HasBlockedImages** property.
- **item:WebClientReadFormQueryString** - Identifies the **WebClientReadFormQueryString** property.
- **item:WebClientEditFormQueryString** - Identifies the **WebClientEditFormQueryString** property.
- **item:IconIndex** - Identifies the **IconIndex** property.

## Message-Specific Properties

### Conversation Properties

- **message:ConversationIndex** - Identifies the **ConversationIndex** property.
- **message:ConversationTopic** - Identifies the **ConversationTopic** property.

### Message Identification

- **message:InternetMessageId** - Identifies the **InternetMessageId** property.
- **message:References** - Identifies the **References** property.

### Read Status

- **message:IsRead** - Identifies the **IsRead** property.
- **message:IsResponseRequested** - Identifies the **IsResponseRequested** property.
- **message:IsReadReceiptRequested** - Identifies the **IsReadReceiptRequested** property.
- **message:IsDeliveryReceiptRequested** - Identifies the **IsDeliveryReceiptRequested** property.

### Recipients

- **message:ToRecipients** - Identifies the **ToRecipients** property.
- **message:CcRecipients** - Identifies the **CcRecipients** property.
- **message:BccRecipients** - Identifies the **BccRecipients** property.

### Sender Information

- **message:From** - Identifies the **From** property.
- **message:Sender** - Identifies the **Sender** property.
- **message:ReplyTo** - Identifies the **ReplyTo** property.

### Received Information

- **message:ReceivedBy** - Identifies the **ReceivedBy** property.
- **message:ReceivedRepresenting** - Identifies the **ReceivedRepresenting** property.

### Special Message Types

- **message:ApprovalRequestData** - Identifies the **ApprovalRequestData** property.
- **message:VotingInformation** - Identifies the **VotingInformation** property.
- **message:ReminderMessageData** - Identifies the **ReminderMessageData** property.

## Meeting-Specific Properties

### Meeting Response Properties

- **meeting:AssociatedCalendarItemId** - Identifies the **AssociatedCalendarItemId** property.
- **meeting:IsDelegated** - Identifies the **IsDelegated** property.
- **meeting:IsOutOfDate** - Identifies the **IsOutOfDate** property.
- **meeting:HasBeenProcessed** - Identifies the **HasBeenProcessed** property.
- **meeting:ResponseType** - Identifies the **ResponseType** property.
- **meeting:ProposedStart** - Identifies the **ProposedStart** property.
- **meeting:ProposedEnd** - Identifies the **ProposedEnd** property.

### Meeting Request Properties

- **meetingRequest:MeetingRequestType** - Identifies the **MeetingRequestType** property.
- **meetingRequest:IntendedFreeBusyStatus** - Identifies the **IntendedFreeBusyStatus** property.
- **meetingRequest:ChangeHighlights** - Identifies the **ChangeHighlights** property.

## Calendar-Specific Properties

### Time Properties

- **calendar:Start** - Identifies the **Start** property.
- **calendar:End** - Identifies the **End** property.
- **calendar:OriginalStart** - Identifies the **OriginalStart** property.
- **calendar:StartWallClock** - Identifies the **StartWallClock** property.
- **calendar:EndWallClock** - Identifies the **EndWallClock** property.
- **calendar:StartTimeZoneId** - Identifies the **StartTimeZoneId** property.
- **calendar:EndTimeZoneId** - Identifies the **EndTimeZoneId** property.

### Calendar Item Properties

- **calendar:IsAllDayEvent** - Identifies the **IsAllDayEvent** property.
- **calendar:LegacyFreeBusyStatus** - Identifies the **LegacyFreeBusyStatus** property.
- **calendar:Location** - Identifies the **Location** property.
- **calendar:When** - Identifies the **When** property.
- **calendar:IsMeeting** - Identifies the **IsMeeting** property.
- **calendar:IsCancelled** - Identifies the **IsCancelled** property.
- **calendar:IsRecurring** - Identifies the **IsRecurring** property.
- **calendar:MeetingRequestWasSent** - Identifies the **MeetingRequestWasSent** property.
- **calendar:IsResponseRequested** - Identifies the **IsResponseRequested** property.
- **calendar:CalendarItemType** - Identifies the **CalendarItemType** property.
- **calendar:MyResponseType** - Identifies the **MyResponseType** property.
- **calendar:Organizer** - Identifies the **Organizer** property.
