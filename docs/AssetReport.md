# AssetReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_report_id** | **String** | A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive. | 
**client_report_id** | Option<**String**> | An identifier you determine and submit for the Asset Report. | 
**date_generated** | **String** | The date and time when the Asset Report was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (e.g. \"2018-04-12T03:32:11Z\"). | 
**days_requested** | **f32** | The duration of transaction history you requested | 
**user** | [**crate::models::AssetReportUser**](AssetReportUser.md) |  | 
**items** | [**Vec<crate::models::AssetReportItem>**](AssetReportItem.md) | Data returned by Plaid about each of the Items included in the Asset Report. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


