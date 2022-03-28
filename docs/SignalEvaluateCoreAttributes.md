# SignalEvaluateCoreAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**unauthorized_transactions_count_7d** | Option<**i32**> | We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 7 days from the account that will be debited. | [optional]
**unauthorized_transactions_count_30d** | Option<**i32**> | We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 30 days from the account that will be debited. | [optional]
**unauthorized_transactions_count_60d** | Option<**i32**> | We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 60 days from the account that will be debited. | [optional]
**unauthorized_transactions_count_90d** | Option<**i32**> | We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 90 days from the account that will be debited. | [optional]
**nsf_overdraft_transactions_count_7d** | Option<**i32**> | We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 7 days from the account that will be debited. | [optional]
**nsf_overdraft_transactions_count_30d** | Option<**i32**> | We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 30 days from the account that will be debited. | [optional]
**nsf_overdraft_transactions_count_60d** | Option<**i32**> | We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 60 days from the account that will be debited. | [optional]
**nsf_overdraft_transactions_count_90d** | Option<**i32**> | We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 90 days from the account that will be debited. | [optional]
**days_since_first_plaid_connection** | Option<**i32**> | The number of days since the first time the Item was connected to an application via Plaid | [optional]
**plaid_connections_count_7d** | Option<**i32**> | The number of times the Item has been connected to applications via Plaid over the past 7 days | [optional]
**plaid_connections_count_30d** | Option<**i32**> | The number of times the Item has been connected to applications via Plaid over the past 30 days | [optional]
**total_plaid_connections_count** | Option<**i32**> | The total number of times the Item has been connected to applications via Plaid | [optional]
**is_savings_or_money_market_account** | Option<**bool**> | Indicates if the ACH transaction funding account is a savings/money market account | [optional]
**total_credit_transactions_amount_10d** | Option<**f64**> | The total credit (inflow) transaction amount over the past 10 days from the account that will be debited | [optional]
**total_debit_transactions_amount_10d** | Option<**f64**> | The total debit (outflow) transaction amount over the past 10 days from the account that will be debited | [optional]
**p50_credit_transactions_amount_28d** | Option<**f64**> | The 50th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited | [optional]
**p50_debit_transactions_amount_28d** | Option<**f64**> | The 50th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited | [optional]
**p95_credit_transactions_amount_28d** | Option<**f64**> | The 95th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited | [optional]
**p95_debit_transactions_amount_28d** | Option<**f64**> | The 95th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited | [optional]
**days_with_negative_balance_count_90d** | Option<**i32**> | The number of days within the past 90 days when the account that will be debited had a negative end-of-day available balance | [optional]
**p90_eod_balance_30d** | Option<**f64**> | The 90th percentile of the end-of-day available balance over the past 30 days of the account that will be debited | [optional]
**p90_eod_balance_60d** | Option<**f64**> | The 90th percentile of the end-of-day available balance over the past 60 days of the account that will be debited | [optional]
**p90_eod_balance_90d** | Option<**f64**> | The 90th percentile of the end-of-day available balance over the past 90 days of the account that will be debited | [optional]
**p10_eod_balance_30d** | Option<**f64**> | The 10th percentile of the end-of-day available balance over the past 30 days of the account that will be debited | [optional]
**p10_eod_balance_60d** | Option<**f64**> | The 10th percentile of the end-of-day available balance over the past 60 days of the account that will be debited | [optional]
**p10_eod_balance_90d** | Option<**f64**> | The 10th percentile of the end-of-day available balance over the past 90 days of the account that will be debited | [optional]
**available_balance** | Option<**f64**> | Available balance, as of the `balance_last_updated` time. The available balance is the current balance less any outstanding holds or debits that have not yet posted to the account. | [optional]
**current_balance** | Option<**f64**> | Current balance, as of the `balance_last_updated` time. The current balance is the total amount of funds in the account. | [optional]
**balance_last_updated** | Option<**String**> | Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the balance for the given account has been updated. | [optional]
**phone_change_count_28d** | Option<**i32**> | The number of times the account's phone numbers on file have changed over the past 28 days | [optional]
**phone_change_count_90d** | Option<**i32**> | The number of times the account's phone numbers on file have changed over the past 90 days | [optional]
**email_change_count_28d** | Option<**i32**> | The number of times the account's email addresses on file have changed over the past 28 days | [optional]
**email_change_count_90d** | Option<**i32**> | The number of times the account's email addresses on file have changed over the past 90 days | [optional]
**address_change_count_28d** | Option<**i32**> | The number of times the account's addresses on file have changed over the past 28 days | [optional]
**address_change_count_90d** | Option<**i32**> | The number of times the account's addresses on file have changed over the past 90 days | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


