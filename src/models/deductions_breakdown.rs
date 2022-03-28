/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeductionsBreakdown : An object representing the deduction line items for the pay period

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeductionsBreakdown {
    /// Raw amount of the deduction
    #[serde(rename = "current_amount", skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<f64>,
    /// Description of the deduction line item
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(rename = "iso_currency_code", skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
    #[serde(
        rename = "unofficial_currency_code",
        skip_serializing_if = "Option::is_none"
    )]
    pub unofficial_currency_code: Option<String>,
    /// The year-to-date amount of the deduction
    #[serde(rename = "ytd_amount", skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
}

impl DeductionsBreakdown {
    /// An object representing the deduction line items for the pay period
    pub fn new() -> DeductionsBreakdown {
        DeductionsBreakdown {
            current_amount: None,
            description: None,
            iso_currency_code: None,
            unofficial_currency_code: None,
            ytd_amount: None,
        }
    }
}
