/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// SignalPersonName : The user's legal name

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignalPersonName {
    /// The user's name prefix (e.g. \"Mr.\")
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// The user's given name. If the user has a one-word name, it should be provided in this field.
    #[serde(rename = "given_name", skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// The user's middle name
    #[serde(rename = "middle_name", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// The user's family name / surname
    #[serde(rename = "family_name", skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    /// The user's name suffix (e.g. \"II\")
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

impl SignalPersonName {
    /// The user's legal name
    pub fn new() -> SignalPersonName {
        SignalPersonName {
            prefix: None,
            given_name: None,
            middle_name: None,
            family_name: None,
            suffix: None,
        }
    }
}
