/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferAuthorizationGuaranteeDecisionRationaleCode : A code representing the reason Plaid declined to guarantee this transfer:  `RETURN_BANK`: The risk of a bank-initiated return (for example, an R01/NSF) is too high to guarantee this transfer.  `RETURN_CUSTOMER`: The risk of a customer-initiated return (for example, a R10/Unauthorized) is too high to guarantee this transfer.  `GUARANTEE_LIMIT_REACHED`: This transfer is low-risk, but Guaranteed ACH has exhausted an internal limit on the number or rate of guarantees that applies to this transfer.  `RISK_ESTIMATE_UNAVAILABLE`: A risk estimate is unavailable for this Item.

/// A code representing the reason Plaid declined to guarantee this transfer:  `RETURN_BANK`: The risk of a bank-initiated return (for example, an R01/NSF) is too high to guarantee this transfer.  `RETURN_CUSTOMER`: The risk of a customer-initiated return (for example, a R10/Unauthorized) is too high to guarantee this transfer.  `GUARANTEE_LIMIT_REACHED`: This transfer is low-risk, but Guaranteed ACH has exhausted an internal limit on the number or rate of guarantees that applies to this transfer.  `RISK_ESTIMATE_UNAVAILABLE`: A risk estimate is unavailable for this Item.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferAuthorizationGuaranteeDecisionRationaleCode {
    #[serde(rename = "RETURN_BANK")]
    RETURNBANK,
    #[serde(rename = "RETURN_CUSTOMER")]
    RETURNCUSTOMER,
    #[serde(rename = "GUARANTEE_LIMIT_REACHED")]
    GUARANTEELIMITREACHED,
    #[serde(rename = "RISK_ESTIMATE_UNAVAILABLE")]
    RISKESTIMATEUNAVAILABLE,
}

impl ToString for TransferAuthorizationGuaranteeDecisionRationaleCode {
    fn to_string(&self) -> String {
        match self {
            Self::RETURNBANK => String::from("RETURN_BANK"),
            Self::RETURNCUSTOMER => String::from("RETURN_CUSTOMER"),
            Self::GUARANTEELIMITREACHED => String::from("GUARANTEE_LIMIT_REACHED"),
            Self::RISKESTIMATEUNAVAILABLE => String::from("RISK_ESTIMATE_UNAVAILABLE"),
        }
    }
}

impl Default for TransferAuthorizationGuaranteeDecisionRationaleCode {
    fn default() -> TransferAuthorizationGuaranteeDecisionRationaleCode {
        Self::RETURNBANK
    }
}
