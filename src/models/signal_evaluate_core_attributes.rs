/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// SignalEvaluateCoreAttributes : The core attributes object contains additional data that can be used to assess the ACH return risk. Examples of data include:  `days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid `plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days `plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days `total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid `is_savings_or_money_market_account`: Indicates whether the ACH transaction funding account is a savings/money market account  For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignalEvaluateCoreAttributes {
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 7 days from the account that will be debited.
    #[serde(
        rename = "unauthorized_transactions_count_7d",
        skip_serializing_if = "Option::is_none"
    )]
    pub unauthorized_transactions_count_7d: Option<i32>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 30 days from the account that will be debited.
    #[serde(
        rename = "unauthorized_transactions_count_30d",
        skip_serializing_if = "Option::is_none"
    )]
    pub unauthorized_transactions_count_30d: Option<i32>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 60 days from the account that will be debited.
    #[serde(
        rename = "unauthorized_transactions_count_60d",
        skip_serializing_if = "Option::is_none"
    )]
    pub unauthorized_transactions_count_60d: Option<i32>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 90 days from the account that will be debited.
    #[serde(
        rename = "unauthorized_transactions_count_90d",
        skip_serializing_if = "Option::is_none"
    )]
    pub unauthorized_transactions_count_90d: Option<i32>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 7 days from the account that will be debited.
    #[serde(
        rename = "nsf_overdraft_transactions_count_7d",
        skip_serializing_if = "Option::is_none"
    )]
    pub nsf_overdraft_transactions_count_7d: Option<i32>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 30 days from the account that will be debited.
    #[serde(
        rename = "nsf_overdraft_transactions_count_30d",
        skip_serializing_if = "Option::is_none"
    )]
    pub nsf_overdraft_transactions_count_30d: Option<i32>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 60 days from the account that will be debited.
    #[serde(
        rename = "nsf_overdraft_transactions_count_60d",
        skip_serializing_if = "Option::is_none"
    )]
    pub nsf_overdraft_transactions_count_60d: Option<i32>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 90 days from the account that will be debited.
    #[serde(
        rename = "nsf_overdraft_transactions_count_90d",
        skip_serializing_if = "Option::is_none"
    )]
    pub nsf_overdraft_transactions_count_90d: Option<i32>,
    /// The number of days since the first time the Item was connected to an application via Plaid
    #[serde(
        rename = "days_since_first_plaid_connection",
        skip_serializing_if = "Option::is_none"
    )]
    pub days_since_first_plaid_connection: Option<i32>,
    /// The number of times the Item has been connected to applications via Plaid over the past 7 days
    #[serde(
        rename = "plaid_connections_count_7d",
        skip_serializing_if = "Option::is_none"
    )]
    pub plaid_connections_count_7d: Option<i32>,
    /// The number of times the Item has been connected to applications via Plaid over the past 30 days
    #[serde(
        rename = "plaid_connections_count_30d",
        skip_serializing_if = "Option::is_none"
    )]
    pub plaid_connections_count_30d: Option<i32>,
    /// The total number of times the Item has been connected to applications via Plaid
    #[serde(
        rename = "total_plaid_connections_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_plaid_connections_count: Option<i32>,
    /// Indicates if the ACH transaction funding account is a savings/money market account
    #[serde(
        rename = "is_savings_or_money_market_account",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_savings_or_money_market_account: Option<bool>,
    /// The total credit (inflow) transaction amount over the past 10 days from the account that will be debited
    #[serde(
        rename = "total_credit_transactions_amount_10d",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_credit_transactions_amount_10d: Option<f64>,
    /// The total debit (outflow) transaction amount over the past 10 days from the account that will be debited
    #[serde(
        rename = "total_debit_transactions_amount_10d",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_debit_transactions_amount_10d: Option<f64>,
    /// The 50th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited
    #[serde(
        rename = "p50_credit_transactions_amount_28d",
        skip_serializing_if = "Option::is_none"
    )]
    pub p50_credit_transactions_amount_28d: Option<f64>,
    /// The 50th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited
    #[serde(
        rename = "p50_debit_transactions_amount_28d",
        skip_serializing_if = "Option::is_none"
    )]
    pub p50_debit_transactions_amount_28d: Option<f64>,
    /// The 95th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited
    #[serde(
        rename = "p95_credit_transactions_amount_28d",
        skip_serializing_if = "Option::is_none"
    )]
    pub p95_credit_transactions_amount_28d: Option<f64>,
    /// The 95th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited
    #[serde(
        rename = "p95_debit_transactions_amount_28d",
        skip_serializing_if = "Option::is_none"
    )]
    pub p95_debit_transactions_amount_28d: Option<f64>,
    /// The number of days within the past 90 days when the account that will be debited had a negative end-of-day available balance
    #[serde(
        rename = "days_with_negative_balance_count_90d",
        skip_serializing_if = "Option::is_none"
    )]
    pub days_with_negative_balance_count_90d: Option<i32>,
    /// The 90th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    #[serde(
        rename = "p90_eod_balance_30d",
        skip_serializing_if = "Option::is_none"
    )]
    pub p90_eod_balance_30d: Option<f64>,
    /// The 90th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    #[serde(
        rename = "p90_eod_balance_60d",
        skip_serializing_if = "Option::is_none"
    )]
    pub p90_eod_balance_60d: Option<f64>,
    /// The 90th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    #[serde(
        rename = "p90_eod_balance_90d",
        skip_serializing_if = "Option::is_none"
    )]
    pub p90_eod_balance_90d: Option<f64>,
    /// The 10th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    #[serde(
        rename = "p10_eod_balance_30d",
        skip_serializing_if = "Option::is_none"
    )]
    pub p10_eod_balance_30d: Option<f64>,
    /// The 10th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    #[serde(
        rename = "p10_eod_balance_60d",
        skip_serializing_if = "Option::is_none"
    )]
    pub p10_eod_balance_60d: Option<f64>,
    /// The 10th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    #[serde(
        rename = "p10_eod_balance_90d",
        skip_serializing_if = "Option::is_none"
    )]
    pub p10_eod_balance_90d: Option<f64>,
    /// Available balance, as of the `balance_last_updated` time. The available balance is the current balance less any outstanding holds or debits that have not yet posted to the account.
    #[serde(rename = "available_balance", skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<f64>,
    /// Current balance, as of the `balance_last_updated` time. The current balance is the total amount of funds in the account.
    #[serde(rename = "current_balance", skip_serializing_if = "Option::is_none")]
    pub current_balance: Option<f64>,
    /// Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the balance for the given account has been updated.
    #[serde(
        rename = "balance_last_updated",
        skip_serializing_if = "Option::is_none"
    )]
    pub balance_last_updated: Option<String>,
    /// The number of times the account's phone numbers on file have changed over the past 28 days
    #[serde(
        rename = "phone_change_count_28d",
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_change_count_28d: Option<i32>,
    /// The number of times the account's phone numbers on file have changed over the past 90 days
    #[serde(
        rename = "phone_change_count_90d",
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_change_count_90d: Option<i32>,
    /// The number of times the account's email addresses on file have changed over the past 28 days
    #[serde(
        rename = "email_change_count_28d",
        skip_serializing_if = "Option::is_none"
    )]
    pub email_change_count_28d: Option<i32>,
    /// The number of times the account's email addresses on file have changed over the past 90 days
    #[serde(
        rename = "email_change_count_90d",
        skip_serializing_if = "Option::is_none"
    )]
    pub email_change_count_90d: Option<i32>,
    /// The number of times the account's addresses on file have changed over the past 28 days
    #[serde(
        rename = "address_change_count_28d",
        skip_serializing_if = "Option::is_none"
    )]
    pub address_change_count_28d: Option<i32>,
    /// The number of times the account's addresses on file have changed over the past 90 days
    #[serde(
        rename = "address_change_count_90d",
        skip_serializing_if = "Option::is_none"
    )]
    pub address_change_count_90d: Option<i32>,
}

impl SignalEvaluateCoreAttributes {
    /// The core attributes object contains additional data that can be used to assess the ACH return risk. Examples of data include:  `days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid `plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days `plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days `total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid `is_savings_or_money_market_account`: Indicates whether the ACH transaction funding account is a savings/money market account  For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager
    pub fn new() -> SignalEvaluateCoreAttributes {
        SignalEvaluateCoreAttributes {
            unauthorized_transactions_count_7d: None,
            unauthorized_transactions_count_30d: None,
            unauthorized_transactions_count_60d: None,
            unauthorized_transactions_count_90d: None,
            nsf_overdraft_transactions_count_7d: None,
            nsf_overdraft_transactions_count_30d: None,
            nsf_overdraft_transactions_count_60d: None,
            nsf_overdraft_transactions_count_90d: None,
            days_since_first_plaid_connection: None,
            plaid_connections_count_7d: None,
            plaid_connections_count_30d: None,
            total_plaid_connections_count: None,
            is_savings_or_money_market_account: None,
            total_credit_transactions_amount_10d: None,
            total_debit_transactions_amount_10d: None,
            p50_credit_transactions_amount_28d: None,
            p50_debit_transactions_amount_28d: None,
            p95_credit_transactions_amount_28d: None,
            p95_debit_transactions_amount_28d: None,
            days_with_negative_balance_count_90d: None,
            p90_eod_balance_30d: None,
            p90_eod_balance_60d: None,
            p90_eod_balance_90d: None,
            p10_eod_balance_30d: None,
            p10_eod_balance_60d: None,
            p10_eod_balance_90d: None,
            available_balance: None,
            current_balance: None,
            balance_last_updated: None,
            phone_change_count_28d: None,
            phone_change_count_90d: None,
            email_change_count_28d: None,
            email_change_count_90d: None,
            address_change_count_28d: None,
            address_change_count_90d: None,
        }
    }
}
