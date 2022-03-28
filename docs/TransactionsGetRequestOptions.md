# TransactionsGetRequestOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_ids** | Option<**Vec<String>**> | A list of `account_ids` to retrieve for the Item  Note: An error will be returned if a provided `account_id` is not associated with the Item. | [optional]
**count** | Option<**i32**> | The number of transactions to fetch. | [optional][default to 100]
**offset** | Option<**i32**> | The number of transactions to skip. The default value is 0. | [optional][default to 0]
**include_original_description** | Option<**bool**> | Include the raw unparsed transaction description from the financial institution. This field is disabled by default. If you need this information in addition to the parsed data provided, contact your Plaid Account Manager. | [optional][default to false]
**include_personal_finance_category_beta** | Option<**bool**> | Please use [`include_personal_finance_category`](https://plaid.com/docs/api/products/#transactions-get-request-options-include-personal-finance-category) instead. | [optional][default to false]
**include_personal_finance_category** | Option<**bool**> | Include the [`personal_finance_category`](https://plaid.com/docs/api/products/#transactions-get-response-transactions-personal-finance-category) object in the response.  See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


