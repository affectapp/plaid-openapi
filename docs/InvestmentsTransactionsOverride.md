# InvestmentsTransactionsOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date** | [**String**](string.md) | Posting date for the transaction. Must be formatted as an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date. | 
**name** | **String** | The institution's description of the transaction. | 
**quantity** | **f64** | The number of units of the security involved in this transaction. Must be positive if the type is a buy and negative if the type is a sell. | 
**price** | **f64** | The price of the security at which this transaction occurred. | 
**fees** | Option<**f64**> | The combined value of all fees applied to this transaction. | [optional]
**_type** | **String** | The type of the investment transaction. Possible values are: `buy`: Buying an investment `sell`: Selling an investment `cash`: Activity that modifies a cash position `fee`: A fee on the account `transfer`: Activity that modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer | 
**currency** | **String** | Either a valid `iso_currency_code` or `unofficial_currency_code` | 
**security** | Option<[**crate::models::SecurityOverride**](SecurityOverride.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


