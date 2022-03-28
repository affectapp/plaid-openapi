# EarningsTotal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_amount** | Option<**f64**> | Total amount of the earnings for this pay period | [optional]
**current_pay** | Option<[**crate::models::Pay**](Pay.md)> |  | [optional]
**ytd_pay** | Option<[**crate::models::Pay**](Pay.md)> |  | [optional]
**hours** | Option<**f32**> | Total number of hours worked for this pay period | [optional]
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null. | [optional]
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s. | [optional]
**ytd_amount** | Option<**f64**> | The total year-to-date amount of the earnings | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


