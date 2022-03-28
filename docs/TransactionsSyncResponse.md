# TransactionsSyncResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**added** | [**Vec<crate::models::Transaction>**](Transaction.md) | Transactions that have been added to the item since `cursor` ordered by ascending last modified time. | 
**modified** | [**Vec<crate::models::Transaction>**](Transaction.md) | Transactions that have been modified on the item since `cursor` ordered by ascending last modified time. | 
**removed** | [**Vec<crate::models::RemovedTransaction>**](RemovedTransaction.md) | Transactions that have been removed from the item since `cursor` ordered by ascending last modified time. | 
**next_cursor** | **String** | Cursor used for fetching any future updates after the latest update provided in this response. | 
**has_more** | **bool** | Represents if more than requested count of transaction updates exist. If true, the additional updates can be fetched by making an additional request with `cursor` set to `next_cursor`. | 
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


