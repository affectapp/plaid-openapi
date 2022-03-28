# BankTransfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Plaid’s unique identifier for a bank transfer. | 
**ach_class** | [**crate::models::AchClass**](ACHClass.md) |  | 
**account_id** | **String** | The account ID that should be credited/debited for this bank transfer. | 
**_type** | [**crate::models::BankTransferType**](BankTransferType.md) |  | 
**user** | [**crate::models::BankTransferUser**](BankTransferUser.md) |  | 
**amount** | **String** | The amount of the bank transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**iso_currency_code** | **String** | The currency of the transfer amount, e.g. \"USD\" | 
**description** | **String** | The description of the transfer. | 
**created** | **String** | The datetime when this bank transfer was created. This will be of the form `2006-01-02T15:04:05Z` | 
**status** | [**crate::models::BankTransferStatus**](BankTransferStatus.md) |  | 
**network** | [**crate::models::BankTransferNetwork**](BankTransferNetwork.md) |  | 
**cancellable** | **bool** | When `true`, you can still cancel this bank transfer. | 
**failure_reason** | Option<[**crate::models::BankTransferFailure**](BankTransferFailure.md)> |  | 
**custom_tag** | Option<**String**> | A string containing the custom tag provided by the client in the create request. Will be null if not provided. | 
**metadata** | Option<**::std::collections::HashMap<String, String>**> | The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: - The JSON values must be Strings (no nested JSON objects allowed) - Only ASCII characters may be used - Maximum of 50 key/value pairs - Maximum key length of 40 characters - Maximum value length of 500 characters  | 
**origination_account_id** | **String** | Plaid’s unique identifier for the origination account that was used for this transfer. | 
**direction** | Option<[**crate::models::BankTransferDirection**](BankTransferDirection.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


