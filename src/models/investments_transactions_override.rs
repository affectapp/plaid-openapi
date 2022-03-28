/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// InvestmentsTransactionsOverride : Specify the list of investments transactions on the account.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvestmentsTransactionsOverride {
    /// Posting date for the transaction. Must be formatted as an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date.
    #[serde(rename = "date")]
    pub date: String,
    /// The institution's description of the transaction.
    #[serde(rename = "name")]
    pub name: String,
    /// The number of units of the security involved in this transaction. Must be positive if the type is a buy and negative if the type is a sell.
    #[serde(rename = "quantity")]
    pub quantity: f64,
    /// The price of the security at which this transaction occurred.
    #[serde(rename = "price")]
    pub price: f64,
    /// The combined value of all fees applied to this transaction.
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Option<f64>,
    /// The type of the investment transaction. Possible values are: `buy`: Buying an investment `sell`: Selling an investment `cash`: Activity that modifies a cash position `fee`: A fee on the account `transfer`: Activity that modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer
    #[serde(rename = "type")]
    pub _type: String,
    /// Either a valid `iso_currency_code` or `unofficial_currency_code`
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "security", skip_serializing_if = "Option::is_none")]
    pub security: Option<Box<crate::models::SecurityOverride>>,
}

impl InvestmentsTransactionsOverride {
    /// Specify the list of investments transactions on the account.
    pub fn new(
        date: String,
        name: String,
        quantity: f64,
        price: f64,
        _type: String,
        currency: String,
    ) -> InvestmentsTransactionsOverride {
        InvestmentsTransactionsOverride {
            date,
            name,
            quantity,
            price,
            fees: None,
            _type,
            currency,
            security: None,
        }
    }
}
