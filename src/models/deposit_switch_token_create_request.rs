/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// DepositSwitchTokenCreateRequest : DepositSwitchTokenCreateRequest defines the request schema for `/deposit_switch/token/create`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DepositSwitchTokenCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The ID of the deposit switch
    #[serde(rename = "deposit_switch_id")]
    pub deposit_switch_id: String,
}

impl DepositSwitchTokenCreateRequest {
    /// DepositSwitchTokenCreateRequest defines the request schema for `/deposit_switch/token/create`
    pub fn new(deposit_switch_id: String) -> DepositSwitchTokenCreateRequest {
        DepositSwitchTokenCreateRequest {
            client_id: None,
            secret: None,
            deposit_switch_id,
        }
    }
}
