# DefaultUpdateWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | **String** | `TRANSACTIONS` | 
**webhook_code** | **String** | `DEFAULT_UPDATE` | 
**error** | Option<[**crate::models::PlaidError**](PlaidError.md)> |  | [optional]
**new_transactions** | **f32** | The number of new transactions detected since the last time this webhook was fired. | 
**item_id** | **String** | The `item_id` of the Item the webhook relates to. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


