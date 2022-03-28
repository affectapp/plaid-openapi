# AssetsRelayWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | **String** | `ASSETS` | 
**webhook_code** | **String** | `RELAY_EVENT` | 
**relay_event** | [**crate::models::RelayEvent**](RelayEvent.md) |  | 
**secondary_client_id** | **String** | The id of the client with whom the Asset Report is being shared. | 
**asset_relay_token** | **String** | The `asset_relay_token` that was created by calling `/asset_report/relay/create. | 
**asset_report_id** | **String** | The `asset_report_id` that can be provided to `/asset_report/relay/get` to retrieve the Asset Report. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


