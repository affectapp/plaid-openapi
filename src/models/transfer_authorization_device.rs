/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferAuthorizationDevice : Information about the device being used to initiate the authorization.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferAuthorizationDevice {
    /// The IP address of the device being used to initiate the authorization.
    #[serde(rename = "ip_address", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The user agent of the device being used to initiate the authorization.
    #[serde(rename = "user_agent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

impl TransferAuthorizationDevice {
    /// Information about the device being used to initiate the authorization.
    pub fn new() -> TransferAuthorizationDevice {
        TransferAuthorizationDevice {
            ip_address: None,
            user_agent: None,
        }
    }
}

