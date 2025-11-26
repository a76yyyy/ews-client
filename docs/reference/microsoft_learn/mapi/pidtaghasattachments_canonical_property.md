# PidTagHasAttachments Canonical Property | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/pidtaghasattachments-canonical-property>

**Applies to**: Outlook 2013 | Outlook 2016

Contains TRUE if a message contains at least one attachment.

## Property Details

| **Property** | **Value** |
|--------------|-----------|
| **Associated properties** | PR_HASATTACH |
| **Identifier** | 0x0E1B |
| **Data type** | PT_BOOLEAN |
| **Area** | Message attachment |

## Remarks

- The message store copies this property from the **MSGFLAG_HASATTACH** flag of the **PR_MESSAGE_FLAGS** ([PidTagMessageFlags](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/pidtagmessageflags-canonical-property)) property.
- A client application can then use **PR_HASATTACH** to sort on message attachments in a message viewer.
- The value this property is updated with the [IMAPIProp::SaveChanges](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/imapiprop-savechanges) method.

## Related Resources

### Protocol Specifications

- [MS-OXCMSG](https://msdn.microsoft.com/library/7fd7ec40-deec-4c06-9493-1bc06b349682%28Office.15%29.aspx) - Specifies the properties and operations that are permissible for email message objects.

### Header Files

- **Mapidefs.h** - Provides data type definitions.
- **Mapitags.h** - Contains definitions of properties listed as alternate names.

## See Also

- [MAPI Properties](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/mapi-properties)
- [MAPI Canonical Properties](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/mapi-canonical-properties)
- [Mapping Canonical Property Names to MAPI Names](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/mapping-canonical-property-names-to-mapi-names)
- [Mapping MAPI Names to Canonical Property Names](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/mapping-mapi-names-to-canonical-property-names)
