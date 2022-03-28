# IncomeVerificationDocumentsDownloadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**income_verification_id** | Option<**String**> | The ID of the verification. | [optional]
**access_token** | Option<**String**> | The access token associated with the Item data is being requested for. | [optional]
**document_id** | Option<**String**> | The document ID to download. If passed, a single document will be returned in the resulting zip file, rather than all document | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


