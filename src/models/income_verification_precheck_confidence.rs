/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationPrecheckConfidence : The confidence that Plaid can support the user in the digital income verification flow instead of requiring a manual paystub upload. One of the following:  `\"HIGH\"`: It is very likely that this user can use the digital income verification flow.  \"`LOW`\": It is unlikely that this user can use the digital income verification flow.  `\"UNKNOWN\"`: It was not possible to determine if the user is supportable with the information passed.

/// The confidence that Plaid can support the user in the digital income verification flow instead of requiring a manual paystub upload. One of the following:  `\"HIGH\"`: It is very likely that this user can use the digital income verification flow.  \"`LOW`\": It is unlikely that this user can use the digital income verification flow.  `\"UNKNOWN\"`: It was not possible to determine if the user is supportable with the information passed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncomeVerificationPrecheckConfidence {
    #[serde(rename = "HIGH")]
    HIGH,
    #[serde(rename = "LOW")]
    LOW,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,

}

impl ToString for IncomeVerificationPrecheckConfidence {
    fn to_string(&self) -> String {
        match self {
            Self::HIGH => String::from("HIGH"),
            Self::LOW => String::from("LOW"),
            Self::UNKNOWN => String::from("UNKNOWN"),
        }
    }
}

impl Default for IncomeVerificationPrecheckConfidence {
    fn default() -> IncomeVerificationPrecheckConfidence {
        Self::HIGH
    }
}




