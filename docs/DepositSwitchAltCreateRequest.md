# DepositSwitchAltCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**target_account** | [**crate::models::DepositSwitchTargetAccount**](DepositSwitchTargetAccount.md) |  | 
**target_user** | [**crate::models::DepositSwitchTargetUser**](DepositSwitchTargetUser.md) |  | 
**options** | Option<[**crate::models::DepositSwitchCreateRequestOptions**](DepositSwitchCreateRequestOptions.md)> |  | [optional]
**country_code** | Option<**String**> | ISO-3166-1 alpha-2 country code standard. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


