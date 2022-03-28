/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// SignalEvaluateResponse : SignalEvaluateResponse defines the response schema for `/signal/income/evaluate`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignalEvaluateResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
    #[serde(rename = "scores")]
    pub scores: crate::models::SignalScores,
    #[serde(rename = "core_attributes", skip_serializing_if = "Option::is_none")]
    pub core_attributes: Option<Box<crate::models::SignalEvaluateCoreAttributes>>,
}

impl SignalEvaluateResponse {
    /// SignalEvaluateResponse defines the response schema for `/signal/income/evaluate`
    pub fn new(request_id: String, scores: crate::models::SignalScores) -> SignalEvaluateResponse {
        SignalEvaluateResponse {
            request_id,
            scores,
            core_attributes: None,
        }
    }
}
