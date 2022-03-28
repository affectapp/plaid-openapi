/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferAuthorization : Contains the authorization decision for a proposed transfer



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferAuthorization {
    /// Plaid’s unique identifier for a transfer authorization.
    #[serde(rename = "id")]
    pub id: String,
    /// The datetime representing when the authorization was created, in the format `2006-01-02T15:04:05Z`.
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "decision")]
    pub decision: crate::models::TransferAuthorizationDecision,
    #[serde(rename = "decision_rationale")]
    pub decision_rationale: Option<crate::models::TransferAuthorizationDecisionRationale>,
    #[serde(rename = "guarantee_decision")]
    pub guarantee_decision: Option<crate::models::TransferAuthorizationGuaranteeDecision>,
    #[serde(rename = "guarantee_decision_rationale")]
    pub guarantee_decision_rationale: Option<crate::models::TransferAuthorizationGuaranteeDecisionRationale>,
    #[serde(rename = "proposed_transfer")]
    pub proposed_transfer: crate::models::TransferAuthorizationProposedTransfer,
}

impl TransferAuthorization {
    /// Contains the authorization decision for a proposed transfer
    pub fn new(id: String, created: String, decision: crate::models::TransferAuthorizationDecision, decision_rationale: Option<crate::models::TransferAuthorizationDecisionRationale>, guarantee_decision: Option<crate::models::TransferAuthorizationGuaranteeDecision>, guarantee_decision_rationale: Option<crate::models::TransferAuthorizationGuaranteeDecisionRationale>, proposed_transfer: crate::models::TransferAuthorizationProposedTransfer) -> TransferAuthorization {
        TransferAuthorization {
            id,
            created,
            decision,
            decision_rationale,
            guarantee_decision,
            guarantee_decision_rationale,
            proposed_transfer,
        }
    }
}


