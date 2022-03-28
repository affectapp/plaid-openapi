# WalletTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_id** | **String** | A unique ID identifying the transaction | 
**reference** | **String** | A reference for the transaction | 
**_type** | **String** | The type of of the transaction. Currently, only `\"PAYOUT\"` is supported. | 
**amount** | [**crate::models::WalletTransactionAmount**](WalletTransactionAmount.md) |  | 
**counterparty** | [**crate::models::WalletTransactionCounterparty**](WalletTransactionCounterparty.md) |  | 
**status** | [**crate::models::WalletTransactionStatus**](WalletTransactionStatus.md) |  | 
**created_at** | **String** | Timestamp when the transaction was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


