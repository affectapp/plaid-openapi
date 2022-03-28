# HoldingsDefaultUpdateWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | **String** | `HOLDINGS` | 
**webhook_code** | **String** | `DEFAULT_UPDATE` | 
**item_id** | **String** | The `item_id` of the Item associated with this webhook, warning, or error | 
**error** | Option<[**crate::models::PlaidError**](PlaidError.md)> |  | [optional]
**new_holdings** | **f32** | The number of new holdings reported since the last time this webhook was fired. | 
**updated_holdings** | **f32** | The number of updated holdings reported since the last time this webhook was fired. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


