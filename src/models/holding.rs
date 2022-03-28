/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// Holding : A securities holding at an institution.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Holding {
    /// The Plaid `account_id` associated with the holding.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The Plaid `security_id` associated with the holding.
    #[serde(rename = "security_id")]
    pub security_id: String,
    /// The last price given by the institution for this security.
    #[serde(rename = "institution_price")]
    pub institution_price: f64,
    /// The date at which `institution_price` was current.
    #[serde(rename = "institution_price_as_of")]
    pub institution_price_as_of: Option<String>,
    /// The value of the holding, as reported by the institution.
    #[serde(rename = "institution_value")]
    pub institution_value: f64,
    /// The cost basis of the holding.
    #[serde(rename = "cost_basis")]
    pub cost_basis: Option<f64>,
    /// The total quantity of the asset held, as reported by the financial institution. If the security is an option, `quantity` will reflect the total number of options (typically the number of contracts multiplied by 100), not the number of contracts.
    #[serde(rename = "quantity")]
    pub quantity: f64,
    /// The ISO-4217 currency code of the holding. Always `null` if `unofficial_currency_code` is non-`null`.
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
    #[serde(rename = "unofficial_currency_code")]
    pub unofficial_currency_code: Option<String>,
}

impl Holding {
    /// A securities holding at an institution.
    pub fn new(
        account_id: String,
        security_id: String,
        institution_price: f64,
        institution_price_as_of: Option<String>,
        institution_value: f64,
        cost_basis: Option<f64>,
        quantity: f64,
        iso_currency_code: Option<String>,
        unofficial_currency_code: Option<String>,
    ) -> Holding {
        Holding {
            account_id,
            security_id,
            institution_price,
            institution_price_as_of,
            institution_value,
            cost_basis,
            quantity,
            iso_currency_code,
            unofficial_currency_code,
        }
    }
}
