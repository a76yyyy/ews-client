# ResponseCode | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsecode>

This document lists the various response codes used in Exchange Web Services (EWS) operations.

## Response Codes

### Success Codes

- **NoError** - No error occurred for the request.

### Error Codes

#### Access and Authentication Errors

- **ErrorAccessDenied** - This error occurs when the calling account does not have the rights to perform the requested action.
- **ErrorAccountDisabled** - This error occurs when the account in question has been disabled.
- **ErrorPasswordChangeRequired** - Password change is required.
- **ErrorPasswordExpired** - Password has expired.

#### Active Directory Errors

- **ErrorADOperation** - This error occurs when the operation failed because of communication problems with Active Directory Domain Services (AD DS).
- **ErrorADSessionFilter** - This error is returned when a **ResolveNames** operation request specifies a name that is not valid.
- **ErrorADUnavailable** - This error occurs when AD DS is unavailable. Try your request again later.
- **ErrorAddressSpaceNotFound** - This error occurs when the address space record, or Domain Name System (DNS) domain name, for cross-forest availability could not be found in the Active Directory database.

#### Attachment Errors

- **ErrorAttachmentNestLevelLimitExceeded** - Specifies that an attempt was made to create an item with more than 10 nested attachments. This value was introduced in Exchange Server 2010 Service Pack 2 (SP2).
- **ErrorAttachmentSizeLimitExceeded** - The [CreateAttachment](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createattachment) element returns this error if an attempt is made to create an attachment with size exceeding Int32.MaxValue, in bytes.

#### Calendar Errors

- **ErrorCalendarCannotMoveOrCopyOccurrence** - This error occurs when an attempt is made to move or copy an occurrence of a recurring calendar item.
- **ErrorCalendarCannotUpdateDeletedItem** - This error occurs when an attempt is made to update a calendar item that is located in the Deleted Items folder.
- **ErrorCalendarDurationIsTooLong** - This error occurs during a **CreateItem** or **UpdateItem** operation when a calendar item duration is longer than the maximum allowed, which is currently 5 years.
- **ErrorCalendarEndDateIsEarlierThanStartDate** - This error occurs when a calendar End time is set to the same time or after the Start time.
- **ErrorCalendarFolderIsInvalidForCalendarView** - This error occurs when the specified folder for a **FindItem** operation with a [CalendarView](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/calendarview) element is not of calendar folder type.

#### Task Errors

- **ErrorAffectedTaskOccurrencesRequired** - This error indicates that the **AffectedTaskOccurrences** attribute was not specified when deleting task items.

#### Connection and Store Errors

- **ErrorConnectionFailed** - Connection to the server failed.
- **ErrorMailboxStoreUnavailable** - The mailbox store is unavailable.
- **ErrorMailboxMoveInProgress** - A mailbox move is in progress.
- **ErrorQuotaExceeded** - Storage quota has been exceeded.
- **ErrorInsufficientResources** - Insufficient resources to complete the operation.

#### Processing Errors

- **ErrorBatchProcessingStopped** - This error indicates that an exception occurred while processing an item and that exception is likely to occur for the items that follow.

#### Autodiscover Errors

- **ErrorAutoDiscoverFailed** - This error indicates that Exchange Web Services tried to determine the location of a cross-forest computer but the call to the Autodiscover service failed.

#### Availability Errors

- **ErrorAvailabilityConfigNotFound** - This error indicates that the availability configuration information for the local forest is missing from AD DS.

#### Archive Errors

- **ErrorArchiveFolderPathCreation** - Indicates an error in archive folder path creation.
- **ErrorArchiveMailboxNotEnabled** - Indicates that the archive mailbox was not enabled.
- **ErrorArchiveMailboxServiceDiscoveryFailed** - Indicates that archive mailbox service discovery failed.

#### Delegate Management Errors

- **ErrorAddDelegatesFailed** - This error occurs when a list with added delegates cannot be saved.

#### Calendar Permission Errors

- **ErrorCalendarIsDelegatedForAccept** - This error indicates that the [AcceptItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/acceptitem) element is invalid for a calendar item or meeting request in a delegated scenario.
- **ErrorCalendarIsDelegatedForDecline** - This error indicates that the [DeclineItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/declineitem) element is invalid for a calendar item or meeting request in a delegated scenario.
- **ErrorCalendarIsDelegatedForRemove** - This error indicates that the [RemoveItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/removeitem) element is invalid for a meeting cancellation in a delegated scenario.
- **ErrorCalendarIsDelegatedForTentative** - This error indicates that the [TentativelyAcceptItem](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/tentativelyacceptitem) element is invalid for a calendar item or meeting request in a delegated scenario.

#### Calendar State Errors

- **ErrorCalendarIsCancelledForAccept** - This error indicates that a calendar item has been canceled.
- **ErrorCalendarIsCancelledForDecline** - This error indicates that a calendar item has been canceled.
- **ErrorCalendarIsCancelledForRemove** - This error indicates that a calendar item has been canceled.
- **ErrorCalendarIsCancelledForTentative** - This error indicates that a calendar item has been canceled.
- **ErrorCalendarIsNotOrganizer** - This error indicates that the operation (currently CancelItem) on the calendar item cannot be performed by someone who is not the organizer.

#### Calendar ID Errors

- **ErrorCalendarCannotUseIdForOccurrenceId** - This error occurs when the UpdateItem, GetItem, DeleteItem, MoveItem, CopyItem, or SendItem operation is called and the ID that was specified is not an occurrence ID of any recurring calendar item.
- **ErrorCalendarCannotUseIdForRecurringMasterId** - This error occurs when the **UpdateItem**, **GetItem**, **DeleteItem**, **MoveItem**, **CopyItem**, or **SendItem** operation is called and the ID that was specified is not an ID of any recurring master item.

#### Calendar Property Errors

- **ErrorCalendarInvalidDayForTimeChangePattern** - This error occurs during a **CreateItem** or **UpdateItem** operation when invalid values of Day, WeekendDay, and Weekday are used to define the time change pattern.
- **ErrorCalendarInvalidDayForWeeklyRecurrence** - This error occurs during a **CreateItem** or **UpdateItem** operation when invalid values of Day, WeekDay, and WeekendDay are used to specify the weekly recurrence.
- **ErrorCalendarInvalidPropertyState** - This error occurs when the state of a calendar item recurrence binary large object (BLOB) in the Exchange store is invalid.
- **ErrorCalendarInvalidRecurrence** - This error occurs when the specified recurrence cannot be created.
- **ErrorCalendarInvalidTimeZone** - This error occurs when an invalid time zone is encountered.

#### Internal Use Only

- **ErrorAccessModeSpecified** - This error is for internal use only. This error is not returned.
- **ErrorCalendarInvalidAttributeValue** - This response code is not used.
- **ErrorCalendarInvalidPropertyValue** - This response code is not used.
- **ErrorIsClutter** - Intended for internal use only.
