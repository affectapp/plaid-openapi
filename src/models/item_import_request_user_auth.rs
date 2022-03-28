/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemImportRequestUserAuth : Object of user ID and auth token pair, permitting Plaid to aggregate a user’s accounts



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemImportRequestUserAuth {
    /// Opaque user identifier
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// Authorization token Plaid will use to aggregate this user’s accounts
    #[serde(rename = "auth_token")]
    pub auth_token: String,
}

impl ItemImportRequestUserAuth {
    /// Object of user ID and auth token pair, permitting Plaid to aggregate a user’s accounts
    pub fn new(user_id: String, auth_token: String) -> ItemImportRequestUserAuth {
        ItemImportRequestUserAuth {
            user_id,
            auth_token,
        }
    }
}


