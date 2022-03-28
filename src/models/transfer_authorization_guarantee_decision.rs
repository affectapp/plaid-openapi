/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferAuthorizationGuaranteeDecision : Indicates whether the transfer is guaranteed by Plaid (Guaranteed ACH customers only). This field will contain either `GUARANTEED` or `NOT_GUARANTEED` indicating whether Plaid will guarantee the transfer. If the transfer is not guaranteed, additional information will be provided in the `guarantee_decision_rationale` field. Refer to the `code` field in `guarantee_decision_rationale` for details.

/// Indicates whether the transfer is guaranteed by Plaid (Guaranteed ACH customers only). This field will contain either `GUARANTEED` or `NOT_GUARANTEED` indicating whether Plaid will guarantee the transfer. If the transfer is not guaranteed, additional information will be provided in the `guarantee_decision_rationale` field. Refer to the `code` field in `guarantee_decision_rationale` for details.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferAuthorizationGuaranteeDecision {
    #[serde(rename = "GUARANTEED")]
    GUARANTEED,
    #[serde(rename = "NOT_GUARANTEED")]
    NOTGUARANTEED,
    #[serde(rename = "null")]
    Null,

}

impl ToString for TransferAuthorizationGuaranteeDecision {
    fn to_string(&self) -> String {
        match self {
            Self::GUARANTEED => String::from("GUARANTEED"),
            Self::NOTGUARANTEED => String::from("NOT_GUARANTEED"),
            Self::Null => String::from("null"),
        }
    }
}

impl Default for TransferAuthorizationGuaranteeDecision {
    fn default() -> TransferAuthorizationGuaranteeDecision {
        Self::GUARANTEED
    }
}




