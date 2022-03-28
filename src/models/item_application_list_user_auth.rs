/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemApplicationListUserAuth : User authentication parameters, for clients making a request without an `access_token`. This is only allowed for select clients and will not be supported in the future. Most clients should call /item/import to obtain an access token before making a request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemApplicationListUserAuth {
    /// Account username.
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Account username hashed by FI.
    #[serde(rename = "fi_username_hash", skip_serializing_if = "Option::is_none")]
    pub fi_username_hash: Option<String>,
}

impl ItemApplicationListUserAuth {
    /// User authentication parameters, for clients making a request without an `access_token`. This is only allowed for select clients and will not be supported in the future. Most clients should call /item/import to obtain an access token before making a request.
    pub fn new() -> ItemApplicationListUserAuth {
        ItemApplicationListUserAuth {
            user_id: None,
            fi_username_hash: None,
        }
    }
}


