# InvestmentsDefaultUpdateWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | **String** | `INVESTMENTS_TRANSACTIONS` | 
**webhook_code** | **String** | `DEFAULT_UPDATE` | 
**item_id** | **String** | The `item_id` of the Item associated with this webhook, warning, or error | 
**error** | Option<[**crate::models::PlaidError**](PlaidError.md)> |  | [optional]
**new_investments_transactions** | **f32** | The number of new transactions reported since the last time this webhook was fired. | 
**canceled_investments_transactions** | **f32** | The number of canceled transactions reported since the last time this webhook was fired. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


