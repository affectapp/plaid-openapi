/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// DepositSwitchCreateRequest : DepositSwitchCreateRequest defines the request schema for `/deposit_switch/create`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DepositSwitchCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Access token for the target Item, typically provided in the Import Item response.
    #[serde(rename = "target_access_token")]
    pub target_access_token: String,
    /// Plaid Account ID that specifies the target bank account. This account will become the recipient for a user's direct deposit.
    #[serde(rename = "target_account_id")]
    pub target_account_id: String,
    /// ISO-3166-1 alpha-2 country code standard.
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<CountryCode>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::DepositSwitchCreateRequestOptions>>,
}

impl DepositSwitchCreateRequest {
    /// DepositSwitchCreateRequest defines the request schema for `/deposit_switch/create`
    pub fn new(
        target_access_token: String,
        target_account_id: String,
    ) -> DepositSwitchCreateRequest {
        DepositSwitchCreateRequest {
            client_id: None,
            secret: None,
            target_access_token,
            target_account_id,
            country_code: None,
            options: None,
        }
    }
}

/// ISO-3166-1 alpha-2 country code standard.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CountryCode {
    #[serde(rename = "US")]
    US,
    #[serde(rename = "CA")]
    CA,
}

impl Default for CountryCode {
    fn default() -> CountryCode {
        Self::US
    }
}
