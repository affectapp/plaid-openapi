# HistoricalBalance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date** | [**String**](string.md) | The date of the calculated historical balance, in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD) | 
**current** | **f64** | The total amount of funds in the account, calculated from the `current` balance in the `balance` object by subtracting inflows and adding back outflows according to the posted date of each transaction.  If the account has any pending transactions, historical balance amounts on or after the date of the earliest pending transaction may differ if retrieved in subsequent Asset Reports as a result of those pending transactions posting. | 
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the balance. Always `null` if `unofficial_currency_code` is non-`null`. | 
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the balance. Always `null` if `iso_currency_code` is non-`null`.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


