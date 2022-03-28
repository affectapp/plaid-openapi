/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HistoricalBalance : An object representing a balance held by an account in the past



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoricalBalance {
    /// The date of the calculated historical balance, in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD)
    #[serde(rename = "date")]
    pub date: String,
    /// The total amount of funds in the account, calculated from the `current` balance in the `balance` object by subtracting inflows and adding back outflows according to the posted date of each transaction.  If the account has any pending transactions, historical balance amounts on or after the date of the earliest pending transaction may differ if retrieved in subsequent Asset Reports as a result of those pending transactions posting.
    #[serde(rename = "current")]
    pub current: f64,
    /// The ISO-4217 currency code of the balance. Always `null` if `unofficial_currency_code` is non-`null`.
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the balance. Always `null` if `iso_currency_code` is non-`null`.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
    #[serde(rename = "unofficial_currency_code")]
    pub unofficial_currency_code: Option<String>,
}

impl HistoricalBalance {
    /// An object representing a balance held by an account in the past
    pub fn new(date: String, current: f64, iso_currency_code: Option<String>, unofficial_currency_code: Option<String>) -> HistoricalBalance {
        HistoricalBalance {
            date,
            current,
            iso_currency_code,
            unofficial_currency_code,
        }
    }
}


