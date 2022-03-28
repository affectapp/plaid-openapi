# InvestmentsTransactionsGetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**access_token** | **String** | The access token associated with the Item data is being requested for. | 
**start_date** | [**String**](string.md) | The earliest date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD. | 
**end_date** | [**String**](string.md) | The most recent date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD. | 
**options** | Option<[**crate::models::InvestmentsTransactionsGetRequestOptions**](InvestmentsTransactionsGetRequestOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


