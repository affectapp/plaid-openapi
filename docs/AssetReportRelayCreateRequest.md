# AssetReportRelayCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**asset_report_token** | **String** | A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report. | 
**secondary_client_id** | **String** | The `secondary_client_id` is the client id of the third party with whom you would like to share the Asset Report. | 
**webhook** | Option<**String**> | URL to which Plaid will send webhooks when the Secondary Client successfully retrieves an Asset Report by calling `asset_report/relay/get`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


