/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EarningsTotal : An object representing both the current pay period and year to date amount for an earning category.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EarningsTotal {
    /// Total amount of the earnings for this pay period
    #[serde(rename = "current_amount", skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<f64>,
    #[serde(rename = "current_pay", skip_serializing_if = "Option::is_none")]
    pub current_pay: Option<crate::models::Pay>,
    #[serde(rename = "ytd_pay", skip_serializing_if = "Option::is_none")]
    pub ytd_pay: Option<crate::models::Pay>,
    /// Total number of hours worked for this pay period
    #[serde(rename = "hours", skip_serializing_if = "Option::is_none")]
    pub hours: Option<f32>,
    /// The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(rename = "iso_currency_code", skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
    #[serde(rename = "unofficial_currency_code", skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
    /// The total year-to-date amount of the earnings
    #[serde(rename = "ytd_amount", skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
}

impl EarningsTotal {
    /// An object representing both the current pay period and year to date amount for an earning category.
    pub fn new() -> EarningsTotal {
        EarningsTotal {
            current_amount: None,
            current_pay: None,
            ytd_pay: None,
            hours: None,
            iso_currency_code: None,
            unofficial_currency_code: None,
            ytd_amount: None,
        }
    }
}


