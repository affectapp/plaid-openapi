# AssetReportItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_id** | **String** | The `item_id` of the Item associated with this webhook, warning, or error | 
**institution_name** | **String** | The full financial institution name associated with the Item. | 
**institution_id** | **String** | The id of the financial institution associated with the Item. | 
**date_last_updated** | **String** | The date and time when this Item’s data was last retrieved from the financial institution, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. | 
**accounts** | [**Vec<crate::models::AccountAssets>**](AccountAssets.md) | Data about each of the accounts open on the Item. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


