# TransferAuthorizationCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**access_token** | **String** | The Plaid `access_token` for the account that will be debited or credited. | 
**account_id** | **String** | The Plaid `account_id` for the account that will be debited or credited. | 
**_type** | [**crate::models::TransferType**](TransferType.md) |  | 
**network** | [**crate::models::TransferNetwork**](TransferNetwork.md) |  | 
**amount** | **String** | The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**ach_class** | [**crate::models::AchClass**](ACHClass.md) |  | 
**user** | [**crate::models::TransferUserInRequest**](TransferUserInRequest.md) |  | 
**device** | Option<[**crate::models::TransferAuthorizationDevice**](TransferAuthorizationDevice.md)> |  | [optional]
**origination_account_id** | Option<**String**> | Plaid's unique identifier for the origination account for this authorization. If not specified, the default account will be used. | [optional]
**iso_currency_code** | Option<**String**> | The currency of the transfer amount. The default value is \"USD\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


