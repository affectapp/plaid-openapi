# CreditBankIncomeSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**income_source_id** | Option<**String**> | A unique identifier for an income source. | [optional]
**income_description** | Option<**String**> | The most common name or original description for the underlying income transactions. | [optional]
**income_category** | Option<[**crate::models::CreditBankIncomeCategory**](CreditBankIncomeCategory.md)> |  | [optional]
**account_id** | Option<**String**> | Plaid's unique idenfier for the account. | [optional]
**start_date** | Option<[**String**](string.md)> | Minimum of all dates within the specific income sources in the user's bank account for days requested by the client. The date will be returned in an ISO 8601 format (YYYY-MM-DD). | [optional]
**end_date** | Option<[**String**](string.md)> | Maximum of all dates within the specific income sources in the user’s bank account for days requested by the client. The date will be returned in an ISO 8601 format (YYYY-MM-DD). | [optional]
**pay_frequency** | Option<[**crate::models::CreditBankIncomePayFrequency**](CreditBankIncomePayFrequency.md)> |  | [optional]
**total_amount** | Option<**f32**> | Total amount of earnings in the user’s bank account for the specific income source for days requested by the client. | [optional]
**transaction_count** | Option<**i32**> | Number of transactions for the income source within the start and end date. | [optional]
**historical_summary** | Option<[**Vec<crate::models::CreditBankIncomeHistoricalSummary>**](CreditBankIncomeHistoricalSummary.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


