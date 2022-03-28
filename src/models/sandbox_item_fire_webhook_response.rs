/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxItemFireWebhookResponse : SandboxItemFireWebhookResponse defines the response schema for `/sandbox/item/fire_webhook`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxItemFireWebhookResponse {
    /// Value is `true`  if the test` webhook_code`  was successfully fired.
    #[serde(rename = "webhook_fired")]
    pub webhook_fired: bool,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl SandboxItemFireWebhookResponse {
    /// SandboxItemFireWebhookResponse defines the response schema for `/sandbox/item/fire_webhook`
    pub fn new(webhook_fired: bool, request_id: String) -> SandboxItemFireWebhookResponse {
        SandboxItemFireWebhookResponse {
            webhook_fired,
            request_id,
        }
    }
}


