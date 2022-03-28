# BankTransferEventListRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**start_date** | Option<**String**> | The start datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`) | [optional]
**end_date** | Option<**String**> | The end datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`) | [optional]
**bank_transfer_id** | Option<**String**> | Plaid’s unique identifier for a bank transfer. | [optional]
**account_id** | Option<**String**> | The account ID to get events for all transactions to/from an account. | [optional]
**bank_transfer_type** | Option<[**crate::models::BankTransferEventListBankTransferType**](BankTransferEventListBankTransferType.md)> |  | [optional]
**event_types** | Option<[**Vec<crate::models::BankTransferEventType>**](BankTransferEventType.md)> | Filter events by event type. | [optional]
**count** | Option<**i32**> | The maximum number of bank transfer events to return. If the number of events matching the above parameters is greater than `count`, the most recent events will be returned. | [optional][default to 25]
**offset** | Option<**i32**> | The offset into the list of bank transfer events. When `count`=25 and `offset`=0, the first 25 events will be returned. When `count`=25 and `offset`=25, the next 25 bank transfer events will be returned. | [optional][default to 0]
**origination_account_id** | Option<**String**> | The origination account ID to get events for transfers from a specific origination account. | [optional]
**direction** | Option<[**crate::models::BankTransferEventListDirection**](BankTransferEventListDirection.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


