# TransactionsRemovedWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | **String** | `TRANSACTIONS` | 
**webhook_code** | **String** | `TRANSACTIONS_REMOVED` | 
**error** | Option<[**crate::models::PlaidError**](PlaidError.md)> |  | [optional]
**removed_transactions** | **Vec<String>** | An array of `transaction_ids` corresponding to the removed transactions | 
**item_id** | **String** | The `item_id` of the Item associated with this webhook, warning, or error | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


