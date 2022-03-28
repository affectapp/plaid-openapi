# TransferEventListRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**start_date** | Option<**String**> | The start datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`) | [optional]
**end_date** | Option<**String**> | The end datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`) | [optional]
**transfer_id** | Option<**String**> | Plaid’s unique identifier for a transfer. | [optional]
**account_id** | Option<**String**> | The account ID to get events for all transactions to/from an account. | [optional]
**transfer_type** | Option<[**crate::models::TransferEventListTransferType**](TransferEventListTransferType.md)> |  | [optional]
**event_types** | Option<[**Vec<crate::models::TransferEventType>**](TransferEventType.md)> | Filter events by event type. | [optional]
**sweep_id** | Option<**String**> | Plaid’s unique identifier for a sweep. | [optional]
**count** | Option<**i32**> | The maximum number of transfer events to return. If the number of events matching the above parameters is greater than `count`, the most recent events will be returned. | [optional][default to 25]
**offset** | Option<**i32**> | The offset into the list of transfer events. When `count`=25 and `offset`=0, the first 25 events will be returned. When `count`=25 and `offset`=25, the next 25 bank transfer events will be returned. | [optional][default to 0]
**origination_account_id** | Option<**String**> | The origination account ID to get events for transfers from a specific origination account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


