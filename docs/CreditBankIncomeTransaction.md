# CreditBankIncomeTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<**f32**> | The settled value of the transaction, denominated in the account's currency as stated in `iso_currency_code` or `unofficial_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, credit card purchases are positive; credit card payment, direct deposits, and refunds are negative. | [optional]
**date** | Option<[**String**](string.md)> | For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted. Both dates are returned in an ISO 8601 format (YYYY-MM-DD). | [optional]
**name** | Option<**String**> | The merchant name or transaction description. | [optional]
**original_description** | Option<**String**> | The string returned by the financial institution to describe the transaction. | [optional]
**pending** | Option<**bool**> | When true, identifies the transaction as pending or unsettled. Pending transaction details (name, type, amount, category ID) may change before they are settled. | [optional]
**transaction_id** | Option<**String**> | The unique ID of the transaction. Like all Plaid identifiers, the `transaction_id` is case sensitive. | [optional]
**check_number** | Option<**String**> | The check number of the transaction. This field is only populated for check transactions. | [optional]
**iso_currency_code** | Option<**String**> | The ISO 4217 currency code of the amount or balance. | [optional]
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


