# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_type** | Option<**String**> | Please use the `payment_channel` field, `transaction_type` will be deprecated in the future.  `digital:` transactions that took place online.  `place:` transactions that were made at a physical location.  `special:` transactions that relate to banks, e.g. fees or deposits.  `unresolved:` transactions that do not fit into the other three types.  | [optional]
**pending_transaction_id** | Option<**String**> | The ID of a posted transaction's associated pending transaction, where applicable. | 
**category_id** | Option<**String**> | The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).  If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights. | 
**category** | Option<**Vec<String>**> | A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).  If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights. | 
**location** | [**crate::models::Location**](Location.md) |  | 
**payment_meta** | [**crate::models::PaymentMeta**](PaymentMeta.md) |  | 
**account_owner** | Option<**String**> | The name of the account owner. This field is not typically populated and only relevant when dealing with sub-accounts. | 
**name** | **String** | The merchant name or transaction description.  If the `transactions` object was returned by a Transactions endpoint such as `/transactions/get`, this field will always appear. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights. | 
**original_description** | Option<**String**> | The string returned by the financial institution to describe the transaction. For transactions returned by `/transactions/get`, this field is in beta and will be omitted unless the client is both enrolled in the closed beta program and has set `options.include_original_description` to `true`. | [optional]
**account_id** | **String** | The ID of the account in which this transaction occurred. | 
**amount** | **f64** | The settled value of the transaction, denominated in the account's currency, as stated in `iso_currency_code` or `unofficial_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative. | 
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-null. | 
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the transaction. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s. | 
**date** | [**String**](string.md) | For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted. Both dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ). | 
**pending** | **bool** | When `true`, identifies the transaction as pending or unsettled. Pending transaction details (name, type, amount, category ID) may change before they are settled. | 
**transaction_id** | **String** | The unique ID of the transaction. Like all Plaid identifiers, the `transaction_id` is case sensitive. | 
**merchant_name** | Option<**String**> | The merchant name, as extracted by Plaid from the `name` field. | [optional]
**check_number** | Option<**String**> | The check number of the transaction. This field is only populated for check transactions. | [optional]
**payment_channel** | **String** | The channel used to make a payment. `online:` transactions that took place online.  `in store:` transactions that were made at a physical location.  `other:` transactions that relate to banks, e.g. fees or deposits.  This field replaces the `transaction_type` field.  | 
**authorized_date** | Option<[**String**](string.md)> | The date that the transaction was authorized. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ). The `authorized_date` field uses machine learning to determine a transaction date for transactions where the `date_transacted` is not available. If the `date_transacted` field is present and not `null`, the `authorized_date` field will have the same value as the `date_transacted` field. | 
**authorized_datetime** | Option<**String**> | Date and time when a transaction was authorized in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).  This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00). | 
**datetime** | Option<**String**> | Date and time when a transaction was posted in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).  This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00). | 
**transaction_code** | Option<[**crate::models::TransactionCode**](TransactionCode.md)> |  | 
**personal_finance_category** | Option<[**crate::models::PersonalFinanceCategory**](PersonalFinanceCategory.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


