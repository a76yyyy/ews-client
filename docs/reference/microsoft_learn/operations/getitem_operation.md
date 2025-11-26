# GetItem operation | Microsoft Learn

**Source:** <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getitem-operation>

Find information about the **GetItem** EWS operation.

The **GetItem** operation gets items from the Exchange store.

## Using the GetItem operation

The **GetItem** operation returns many item properties. The properties that are returned in a **GetItem** response vary based on the requested shape, requested additional properties, and the type of item returned.

[Message](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref) elements represent email messages and all other items that are not strongly typed by the Exchange Web Services (EWS) schema. Items such as IPM.Sharing and IPM.InfoPath are returned as [Message](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref) elements. Exchange does not return the base [Item](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/item) element in responses.

The **GetItem** operation does not return attachments. It does return metadata about an attached item or file. To return an attachment, use the [GetAttachment operation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getattachment-operation).

## GetItem operation SOAP headers

The **GetItem** operation can use the SOAP headers that are listed in the following table.

| ****Header**** | ****Element**** | ****Description**** |
|----------------|-----------------|---------------------|
| **DateTimePrecision** | [DateTimePrecision](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/datetimeprecision) | Specifies the resolution of data/time values in responses from the server, either in seconds or in milliseconds. |
| **Impersonation** | [ExchangeImpersonation](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/exchangeimpersonation) | Identifies the user whom the client application is impersonating. |
| **MailboxCulture** | [MailboxCulture](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailboxculture) | Identifies the culture, as defined in RFC 3066, "Tags for the Identification of Languages", to be used to access the mailbox. |
| **RequestVersion** | [RequestServerVersion](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/requestserverversion) | Identifies the schema version for the operation request. |
| **ServerVersion** | [ServerVersionInfo](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversioninfo) | Identifies the version of the server that responded to the request. |
| **TimeZoneContext** | [TimeZoneContext](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/timezonecontext) | Identifies the time zone to be used for all responses from the server. |

## In This Section

* [GetItem operation (email message)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getitem-operation-email-message)

* [GetItem operation (calendar item)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getitem-operation-calendar-item)

* [GetItem operation (task)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getitem-operation-task)

* [GetItem operation (contact)](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getitem-operation-contact)

## See also

* [EWS reference for Exchange](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-reference-for-exchange)
