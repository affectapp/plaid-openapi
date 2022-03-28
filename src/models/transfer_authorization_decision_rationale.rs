/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferAuthorizationDecisionRationale : The rationale for Plaid's decision regarding a proposed transfer. It is always set for `declined` decisions, and may or may not be null for `approved` decisions.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferAuthorizationDecisionRationale {
    #[serde(rename = "code")]
    pub code: crate::models::TransferAuthorizationDecisionRationaleCode,
    /// A human-readable description of the code associated with a transfer approval or transfer decline.
    #[serde(rename = "description")]
    pub description: String,
}

impl TransferAuthorizationDecisionRationale {
    /// The rationale for Plaid's decision regarding a proposed transfer. It is always set for `declined` decisions, and may or may not be null for `approved` decisions.
    pub fn new(
        code: crate::models::TransferAuthorizationDecisionRationaleCode,
        description: String,
    ) -> TransferAuthorizationDecisionRationale {
        TransferAuthorizationDecisionRationale { code, description }
    }
}
