# ClientProvidedEnhancedTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique transaction identifier to tie transactions back to clients' systems. | 
**description** | **String** | The raw description of the transaction. | 
**amount** | **f64** | The value of the transaction, denominated in the account's currency, as stated in `iso_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative. | 
**iso_currency_code** | **String** | The ISO-4217 currency code of the transaction. | 
**enhancements** | [**crate::models::Enhancements**](Enhancements.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


