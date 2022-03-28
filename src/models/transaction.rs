/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Transaction : A representation of a transaction



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Transaction {
    /// Please use the `payment_channel` field, `transaction_type` will be deprecated in the future.  `digital:` transactions that took place online.  `place:` transactions that were made at a physical location.  `special:` transactions that relate to banks, e.g. fees or deposits.  `unresolved:` transactions that do not fit into the other three types. 
    #[serde(rename = "transaction_type", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,
    /// The ID of a posted transaction's associated pending transaction, where applicable.
    #[serde(rename = "pending_transaction_id")]
    pub pending_transaction_id: Option<String>,
    /// The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).  If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.
    #[serde(rename = "category_id")]
    pub category_id: Option<String>,
    /// A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).  If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.
    #[serde(rename = "category")]
    pub category: Option<Vec<String>>,
    #[serde(rename = "location")]
    pub location: crate::models::Location,
    #[serde(rename = "payment_meta")]
    pub payment_meta: crate::models::PaymentMeta,
    /// The name of the account owner. This field is not typically populated and only relevant when dealing with sub-accounts.
    #[serde(rename = "account_owner")]
    pub account_owner: Option<String>,
    /// The merchant name or transaction description.  If the `transactions` object was returned by a Transactions endpoint such as `/transactions/get`, this field will always appear. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.
    #[serde(rename = "name")]
    pub name: String,
    /// The string returned by the financial institution to describe the transaction. For transactions returned by `/transactions/get`, this field is in beta and will be omitted unless the client is both enrolled in the closed beta program and has set `options.include_original_description` to `true`.
    #[serde(rename = "original_description", skip_serializing_if = "Option::is_none")]
    pub original_description: Option<String>,
    /// The ID of the account in which this transaction occurred.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The settled value of the transaction, denominated in the account's currency, as stated in `iso_currency_code` or `unofficial_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the transaction. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
    #[serde(rename = "unofficial_currency_code")]
    pub unofficial_currency_code: Option<String>,
    /// For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted. Both dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ).
    #[serde(rename = "date")]
    pub date: String,
    /// When `true`, identifies the transaction as pending or unsettled. Pending transaction details (name, type, amount, category ID) may change before they are settled.
    #[serde(rename = "pending")]
    pub pending: bool,
    /// The unique ID of the transaction. Like all Plaid identifiers, the `transaction_id` is case sensitive.
    #[serde(rename = "transaction_id")]
    pub transaction_id: String,
    /// The merchant name, as extracted by Plaid from the `name` field.
    #[serde(rename = "merchant_name", skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /// The check number of the transaction. This field is only populated for check transactions.
    #[serde(rename = "check_number", skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    /// The channel used to make a payment. `online:` transactions that took place online.  `in store:` transactions that were made at a physical location.  `other:` transactions that relate to banks, e.g. fees or deposits.  This field replaces the `transaction_type` field. 
    #[serde(rename = "payment_channel")]
    pub payment_channel: PaymentChannel,
    /// The date that the transaction was authorized. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ). The `authorized_date` field uses machine learning to determine a transaction date for transactions where the `date_transacted` is not available. If the `date_transacted` field is present and not `null`, the `authorized_date` field will have the same value as the `date_transacted` field.
    #[serde(rename = "authorized_date")]
    pub authorized_date: Option<String>,
    /// Date and time when a transaction was authorized in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).  This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00).
    #[serde(rename = "authorized_datetime")]
    pub authorized_datetime: Option<String>,
    /// Date and time when a transaction was posted in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).  This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00).
    #[serde(rename = "datetime")]
    pub datetime: Option<String>,
    #[serde(rename = "transaction_code")]
    pub transaction_code: Option<crate::models::TransactionCode>,
    #[serde(rename = "personal_finance_category", skip_serializing_if = "Option::is_none")]
    pub personal_finance_category: Option<crate::models::PersonalFinanceCategory>,
}

impl Transaction {
    /// A representation of a transaction
    pub fn new(pending_transaction_id: Option<String>, category_id: Option<String>, category: Option<Vec<String>>, location: crate::models::Location, payment_meta: crate::models::PaymentMeta, account_owner: Option<String>, name: String, account_id: String, amount: f64, iso_currency_code: Option<String>, unofficial_currency_code: Option<String>, date: String, pending: bool, transaction_id: String, payment_channel: PaymentChannel, authorized_date: Option<String>, authorized_datetime: Option<String>, datetime: Option<String>, transaction_code: Option<crate::models::TransactionCode>) -> Transaction {
        Transaction {
            transaction_type: None,
            pending_transaction_id,
            category_id,
            category,
            location,
            payment_meta,
            account_owner,
            name,
            original_description: None,
            account_id,
            amount,
            iso_currency_code,
            unofficial_currency_code,
            date,
            pending,
            transaction_id,
            merchant_name: None,
            check_number: None,
            payment_channel,
            authorized_date,
            authorized_datetime,
            datetime,
            transaction_code,
            personal_finance_category: None,
        }
    }
}

/// Please use the `payment_channel` field, `transaction_type` will be deprecated in the future.  `digital:` transactions that took place online.  `place:` transactions that were made at a physical location.  `special:` transactions that relate to banks, e.g. fees or deposits.  `unresolved:` transactions that do not fit into the other three types. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "digital")]
    Digital,
    #[serde(rename = "place")]
    Place,
    #[serde(rename = "special")]
    Special,
    #[serde(rename = "unresolved")]
    Unresolved,
}

impl Default for TransactionType {
    fn default() -> TransactionType {
        Self::Digital
    }
}
/// The channel used to make a payment. `online:` transactions that took place online.  `in store:` transactions that were made at a physical location.  `other:` transactions that relate to banks, e.g. fees or deposits.  This field replaces the `transaction_type` field. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentChannel {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "in store")]
    InStore,
    #[serde(rename = "other")]
    Other,
}

impl Default for PaymentChannel {
    fn default() -> PaymentChannel {
        Self::Online
    }
}

