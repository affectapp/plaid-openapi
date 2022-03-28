/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxItemFireWebhookRequest : SandboxItemFireWebhookRequest defines the request schema for `/sandbox/item/fire_webhook`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxItemFireWebhookRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// The following values for `webhook_code` are supported:  * `DEFAULT_UPDATE` * `NEW_ACCOUNTS_AVAILABLE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: WebhookCode,
}

impl SandboxItemFireWebhookRequest {
    /// SandboxItemFireWebhookRequest defines the request schema for `/sandbox/item/fire_webhook`
    pub fn new(access_token: String, webhook_code: WebhookCode) -> SandboxItemFireWebhookRequest {
        SandboxItemFireWebhookRequest {
            client_id: None,
            secret: None,
            access_token,
            webhook_code,
        }
    }
}

/// The following values for `webhook_code` are supported:  * `DEFAULT_UPDATE` * `NEW_ACCOUNTS_AVAILABLE`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebhookCode {
    #[serde(rename = "DEFAULT_UPDATE")]
    DEFAULTUPDATE,
    #[serde(rename = "NEW_ACCOUNTS_AVAILABLE")]
    NEWACCOUNTSAVAILABLE,
}

impl Default for WebhookCode {
    fn default() -> WebhookCode {
        Self::DEFAULTUPDATE
    }
}
