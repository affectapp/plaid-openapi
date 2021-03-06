/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// Email : An object representing an email address

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Email {
    /// The email address.
    #[serde(rename = "data")]
    pub data: String,
    /// When `true`, identifies the email address as the primary email on an account.
    #[serde(rename = "primary")]
    pub primary: bool,
    /// The type of email account as described by the financial institution.
    #[serde(rename = "type")]
    pub _type: Type,
}

impl Email {
    /// An object representing an email address
    pub fn new(data: String, primary: bool, _type: Type) -> Email {
        Email {
            data,
            primary,
            _type,
        }
    }
}

/// The type of email account as described by the financial institution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "other")]
    Other,
}

impl Default for Type {
    fn default() -> Type {
        Self::Primary
    }
}
