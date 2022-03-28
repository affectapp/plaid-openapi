# IncomeVerificationCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**webhook** | **String** | The URL endpoint to which Plaid should send webhooks related to the progress of the income verification process. | 
**precheck_id** | Option<**String**> | The ID of a precheck created with `/income/verification/precheck`. Will be used to improve conversion of the income verification flow. | [optional]
**options** | Option<[**crate::models::IncomeVerificationCreateRequestOptions**](IncomeVerificationCreateRequestOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


