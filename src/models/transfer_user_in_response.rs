/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferUserInResponse : The legal name and other information for the account holder.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferUserInResponse {
    /// The user's legal name.
    #[serde(rename = "legal_name")]
    pub legal_name: String,
    /// The user's phone number.
    #[serde(rename = "phone_number")]
    pub phone_number: Option<String>,
    /// The user's email address.
    #[serde(rename = "email_address")]
    pub email_address: Option<String>,
    #[serde(rename = "address")]
    pub address: Option<crate::models::TransferUserAddressInResponse>,
}

impl TransferUserInResponse {
    /// The legal name and other information for the account holder.
    pub fn new(
        legal_name: String,
        phone_number: Option<String>,
        email_address: Option<String>,
        address: Option<crate::models::TransferUserAddressInResponse>,
    ) -> TransferUserInResponse {
        TransferUserInResponse {
            legal_name,
            phone_number,
            email_address,
            address,
        }
    }
}
