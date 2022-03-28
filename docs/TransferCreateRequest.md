# TransferCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**idempotency_key** | Option<**String**> | Deprecated. `authorization_id` is now for used idempotency instead.  A random key provided by the client, per unique transfer. Maximum of 50 characters.  The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single transfer is created. | [optional]
**access_token** | **String** | The Plaid `access_token` for the account that will be debited or credited. | 
**account_id** | **String** | The Plaid `account_id` for the account that will be debited or credited. | 
**authorization_id** | **String** | Plaid’s unique identifier for a transfer authorization. This parameter also serves the purpose of acting as an idempotency identifier. | 
**_type** | [**crate::models::TransferType**](TransferType.md) |  | 
**network** | [**crate::models::TransferNetwork**](TransferNetwork.md) |  | 
**amount** | **String** | The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**description** | **String** | The transfer description. Maximum of 10 characters. | 
**ach_class** | [**crate::models::AchClass**](ACHClass.md) |  | 
**user** | [**crate::models::TransferUserInRequest**](TransferUserInRequest.md) |  | 
**metadata** | Option<**::std::collections::HashMap<String, String>**> | The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: - The JSON values must be Strings (no nested JSON objects allowed) - Only ASCII characters may be used - Maximum of 50 key/value pairs - Maximum key length of 40 characters - Maximum value length of 500 characters  | [optional]
**origination_account_id** | Option<**String**> | Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified. Otherwise, this field should be left blank. | [optional]
**iso_currency_code** | Option<**String**> | The currency of the transfer amount. The default value is \"USD\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


