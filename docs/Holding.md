# Holding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **String** | The Plaid `account_id` associated with the holding. | 
**security_id** | **String** | The Plaid `security_id` associated with the holding. | 
**institution_price** | **f64** | The last price given by the institution for this security. | 
**institution_price_as_of** | Option<[**String**](string.md)> | The date at which `institution_price` was current. | 
**institution_value** | **f64** | The value of the holding, as reported by the institution. | 
**cost_basis** | Option<**f64**> | The cost basis of the holding. | 
**quantity** | **f64** | The total quantity of the asset held, as reported by the financial institution. If the security is an option, `quantity` will reflect the total number of options (typically the number of contracts multiplied by 100), not the number of contracts. | 
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the holding. Always `null` if `unofficial_currency_code` is non-`null`. | 
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


