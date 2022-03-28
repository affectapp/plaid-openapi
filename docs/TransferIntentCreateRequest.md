# TransferIntentCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | **String** | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | 
**secret** | **String** | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | 
**account_id** | Option<**String**> | The Plaid `account_id` for the account that will be debited or credited. | [optional]
**mode** | [**crate::models::TransferIntentCreateMode**](TransferIntentCreateMode.md) |  | 
**amount** | **String** | The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**description** | **String** | A description for the underlying transfer. Maximum of 8 characters. | 
**ach_class** | [**crate::models::AchClass**](ACHClass.md) |  | 
**origination_account_id** | Option<**String**> | Plaid’s unique identifier for the origination account for the intent. If not provided, the default account will be used. | [optional]
**user** | [**crate::models::TransferUserInRequest**](TransferUserInRequest.md) |  | 
**metadata** | Option<**::std::collections::HashMap<String, String>**> | The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: - The JSON values must be Strings (no nested JSON objects allowed) - Only ASCII characters may be used - Maximum of 50 key/value pairs - Maximum key length of 40 characters - Maximum value length of 500 characters  | [optional]
**iso_currency_code** | Option<**String**> | The currency of the transfer amount, e.g. \"USD\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


