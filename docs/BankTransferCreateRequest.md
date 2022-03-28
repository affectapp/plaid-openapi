# BankTransferCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**idempotency_key** | **String** | A random key provided by the client, per unique bank transfer. Maximum of 50 characters.  The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a bank transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single bank transfer is created. | 
**access_token** | **String** | The Plaid `access_token` for the account that will be debited or credited. | 
**account_id** | **String** | The Plaid `account_id` for the account that will be debited or credited. | 
**_type** | [**crate::models::BankTransferType**](BankTransferType.md) |  | 
**network** | [**crate::models::BankTransferNetwork**](BankTransferNetwork.md) |  | 
**amount** | **String** | The amount of the bank transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**iso_currency_code** | **String** | The currency of the transfer amount – should be set to \"USD\". | 
**description** | **String** | The transfer description. Maximum of 10 characters. | 
**ach_class** | Option<[**crate::models::AchClass**](ACHClass.md)> |  | [optional]
**user** | [**crate::models::BankTransferUser**](BankTransferUser.md) |  | 
**custom_tag** | Option<**String**> | An arbitrary string provided by the client for storage with the bank transfer. May be up to 100 characters. | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> | The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: - The JSON values must be Strings (no nested JSON objects allowed) - Only ASCII characters may be used - Maximum of 50 key/value pairs - Maximum key length of 40 characters - Maximum value length of 500 characters  | [optional]
**origination_account_id** | Option<**String**> | Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified. Otherwise, this field should be left blank. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


