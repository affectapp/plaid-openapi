/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationPrecheckResponse : IncomeVerificationPrecheckResponse defines the response schema for `/income/verification/precheck`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckResponse {
    /// ID of the precheck. Provide this value when calling `/link/token/create` in order to optimize Link conversion.
    #[serde(rename = "precheck_id")]
    pub precheck_id: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
    #[serde(rename = "confidence")]
    pub confidence: crate::models::IncomeVerificationPrecheckConfidence,
}

impl IncomeVerificationPrecheckResponse {
    /// IncomeVerificationPrecheckResponse defines the response schema for `/income/verification/precheck`.
    pub fn new(precheck_id: String, request_id: String, confidence: crate::models::IncomeVerificationPrecheckConfidence) -> IncomeVerificationPrecheckResponse {
        IncomeVerificationPrecheckResponse {
            precheck_id,
            request_id,
            confidence,
        }
    }
}


