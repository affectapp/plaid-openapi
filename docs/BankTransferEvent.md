# BankTransferEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **i32** | Plaid’s unique identifier for this event. IDs are sequential unsigned 64-bit integers. | 
**timestamp** | **String** | The datetime when this event occurred. This will be of the form `2006-01-02T15:04:05Z`. | 
**event_type** | [**crate::models::BankTransferEventType**](BankTransferEventType.md) |  | 
**account_id** | **String** | The account ID associated with the bank transfer. | 
**bank_transfer_id** | **String** | Plaid’s unique identifier for a bank transfer. | 
**origination_account_id** | Option<**String**> | The ID of the origination account that this balance belongs to. | 
**bank_transfer_type** | [**crate::models::BankTransferType**](BankTransferType.md) |  | 
**bank_transfer_amount** | **String** | The bank transfer amount. | 
**bank_transfer_iso_currency_code** | **String** | The currency of the bank transfer amount. | 
**failure_reason** | Option<[**crate::models::BankTransferFailure**](BankTransferFailure.md)> |  | 
**direction** | Option<[**crate::models::BankTransferDirection**](BankTransferDirection.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


