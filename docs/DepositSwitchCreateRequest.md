# DepositSwitchCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**target_access_token** | **String** | Access token for the target Item, typically provided in the Import Item response.  | 
**target_account_id** | **String** | Plaid Account ID that specifies the target bank account. This account will become the recipient for a user's direct deposit. | 
**country_code** | Option<**String**> | ISO-3166-1 alpha-2 country code standard. | [optional]
**options** | Option<[**crate::models::DepositSwitchCreateRequestOptions**](DepositSwitchCreateRequestOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


