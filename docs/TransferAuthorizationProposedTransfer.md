# TransferAuthorizationProposedTransfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ach_class** | [**crate::models::AchClass**](ACHClass.md) |  | 
**account_id** | **String** | The Plaid `account_id` for the account that will be debited or credited. | 
**_type** | [**crate::models::TransferType**](TransferType.md) |  | 
**user** | [**crate::models::TransferUserInResponse**](TransferUserInResponse.md) |  | 
**amount** | **String** | The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**network** | **String** | The network or rails used for the transfer. | 
**origination_account_id** | **String** | Plaid's unique identifier for the origination account that was used for this transfer. | 
**iso_currency_code** | **String** | The currency of the transfer amount. The default value is \"USD\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


