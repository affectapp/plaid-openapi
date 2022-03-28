# IdentityDefaultUpdateWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | **String** | `IDENTITY` | 
**webhook_code** | **String** | `DEFAULT_UPDATE` | 
**item_id** | **String** | The `item_id` of the Item associated with this webhook, warning, or error | 
**account_ids_with_updated_identity** | [**::std::collections::HashMap<String, Vec<crate::models::IdentityUpdateTypes>>**](array.md) | An object with keys of `account_id`'s that are mapped to their respective identity attributes that changed.  Example: `{ \"XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58\": [\"PHONES\"] }`  | 
**error** | [**crate::models::PlaidError**](PlaidError.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


