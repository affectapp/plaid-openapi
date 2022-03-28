/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// DepositSwitchAltCreateRequest : DepositSwitchAltCreateRequest defines the request schema for `/deposit_switch/alt/create`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DepositSwitchAltCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "target_account")]
    pub target_account: crate::models::DepositSwitchTargetAccount,
    #[serde(rename = "target_user")]
    pub target_user: crate::models::DepositSwitchTargetUser,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::DepositSwitchCreateRequestOptions>>,
    /// ISO-3166-1 alpha-2 country code standard.
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<CountryCode>,
}

impl DepositSwitchAltCreateRequest {
    /// DepositSwitchAltCreateRequest defines the request schema for `/deposit_switch/alt/create`
    pub fn new(
        target_account: crate::models::DepositSwitchTargetAccount,
        target_user: crate::models::DepositSwitchTargetUser,
    ) -> DepositSwitchAltCreateRequest {
        DepositSwitchAltCreateRequest {
            client_id: None,
            secret: None,
            target_account,
            target_user,
            options: None,
            country_code: None,
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
