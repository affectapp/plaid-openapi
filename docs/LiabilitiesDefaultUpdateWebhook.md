# LiabilitiesDefaultUpdateWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | **String** | `LIABILITIES` | 
**webhook_code** | **String** | `DEFAULT_UPDATE` | 
**item_id** | **String** | The `item_id` of the Item associated with this webhook, warning, or error | 
**error** | [**crate::models::PlaidError**](PlaidError.md) |  | 
**account_ids_with_new_liabilities** | **Vec<String>** | An array of `account_id`'s for accounts that contain new liabilities.' | 
**account_ids_with_updated_liabilities** | [**::std::collections::HashMap<String, Vec<String>>**](array.md) | An object with keys of `account_id`'s that are mapped to their respective liabilities fields that changed.  Example: `{ \"XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58\": [\"past_amount_due\"] }`  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


