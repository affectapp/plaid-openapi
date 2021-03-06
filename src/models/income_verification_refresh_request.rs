/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationRefreshRequest : IncomeVerificationRefreshRequest defines the request schema for `/income/verification/refresh`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncomeVerificationRefreshRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The ID of the verification.
    #[serde(
        rename = "income_verification_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub income_verification_id: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}

impl IncomeVerificationRefreshRequest {
    /// IncomeVerificationRefreshRequest defines the request schema for `/income/verification/refresh`
    pub fn new() -> IncomeVerificationRefreshRequest {
        IncomeVerificationRefreshRequest {
            client_id: None,
            secret: None,
            income_verification_id: None,
            access_token: None,
        }
    }
}
