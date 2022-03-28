# HoldingsOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**institution_price** | **f64** | The last price given by the institution for this security | 
**institution_price_as_of** | Option<[**String**](string.md)> | The date at which `institution_price` was current. Must be formatted as an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date. | [optional]
**cost_basis** | Option<**f64**> | The average original value of the holding. Multiple cost basis values for the same security purchased at different prices are not supported. | [optional]
**quantity** | **f64** | The total quantity of the asset held, as reported by the financial institution. | 
**currency** | **String** | Either a valid `iso_currency_code` or `unofficial_currency_code` | 
**security** | [**crate::models::SecurityOverride**](SecurityOverride.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


