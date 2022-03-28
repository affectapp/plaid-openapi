# TransactionsRecurringGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inflow_streams** | [**Vec<crate::models::TransactionStream>**](TransactionStream.md) | An array of depository transaction streams. | 
**outflow_streams** | [**Vec<crate::models::TransactionStream>**](TransactionStream.md) | An array of expense transaction streams. | 
**updated_datetime** | **String** | Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the last time transaction streams for the given account were updated on | 
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


