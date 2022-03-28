/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VerificationAttribute : Details about a certain reason as to why a document could potentially be fraudulent



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VerificationAttribute {
    /// Message indicating the reason as to why the verification failed
    #[serde(rename = "type")]
    pub _type: Option<Type>,
}

impl VerificationAttribute {
    /// Details about a certain reason as to why a document could potentially be fraudulent
    pub fn new(_type: Option<Type>) -> VerificationAttribute {
        VerificationAttribute {
            _type,
        }
    }
}

/// Message indicating the reason as to why the verification failed
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "VERIFICATION_ATTRIBUTE_TYPE_UNKNOWN")]
    VERIFICATIONATTRIBUTETYPEUNKNOWN,
    #[serde(rename = "VERIFICATION_ATTRIBUTE_TYPE_AMOUNT_MATCH")]
    VERIFICATIONATTRIBUTETYPEAMOUNTMATCH,
    #[serde(rename = "VERIFICATION_ATTRIBUTE_TYPE_DATE_MATCH")]
    VERIFICATIONATTRIBUTETYPEDATEMATCH,
    #[serde(rename = "VERIFICATION_ATTRIBUTE_TYPE_DATE_MISMATCH")]
    VERIFICATIONATTRIBUTETYPEDATEMISMATCH,
    #[serde(rename = "VERIFICATION_ATTRIBUTE_TYPE_FILE_TAMPERING")]
    VERIFICATIONATTRIBUTETYPEFILETAMPERING,
    #[serde(rename = "VERIFICATION_ATTRIBUTE_TYPE_DESCRIPTION_MATCH")]
    VERIFICATIONATTRIBUTETYPEDESCRIPTIONMATCH,
    #[serde(rename = "VERIFICATION_ATTRIBUTE_TYPE_DESCRIPTION_MISMATCH")]
    VERIFICATIONATTRIBUTETYPEDESCRIPTIONMISMATCH,
    #[serde(rename = "VERIFICATION_ATTRIBUTE_TYPE_FIRST_NAME_MATCH")]
    VERIFICATIONATTRIBUTETYPEFIRSTNAMEMATCH,
    #[serde(rename = "VERIFICATION_ATTRIBUTE_TYPE_FIRST_NAME_MISMATCH")]
    VERIFICATIONATTRIBUTETYPEFIRSTNAMEMISMATCH,
    #[serde(rename = "VERIFICATION_ATTRIBUTE_TYPE_LAST_NAME_MATCH")]
    VERIFICATIONATTRIBUTETYPELASTNAMEMATCH,
    #[serde(rename = "VERIFICATION_ATTRIBUTE_TYPE_LAST_NAME_MISMATCH")]
    VERIFICATIONATTRIBUTETYPELASTNAMEMISMATCH,
    #[serde(rename = "null")]
    Null,
}

impl Default for Type {
    fn default() -> Type {
        Self::VERIFICATIONATTRIBUTETYPEUNKNOWN
    }
}
