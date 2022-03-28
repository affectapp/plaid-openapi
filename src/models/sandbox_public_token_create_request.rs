/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxPublicTokenCreateRequest : SandboxPublicTokenCreateRequest defines the request schema for `/sandbox/public_token/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The ID of the institution the Item will be associated with
    #[serde(rename = "institution_id")]
    pub institution_id: String,
    /// The products to initially pull for the Item. May be any products that the specified `institution_id`  supports. This array may not be empty.
    #[serde(rename = "initial_products")]
    pub initial_products: Vec<crate::models::Products>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::SandboxPublicTokenCreateRequestOptions>>,
}

impl SandboxPublicTokenCreateRequest {
    /// SandboxPublicTokenCreateRequest defines the request schema for `/sandbox/public_token/create`
    pub fn new(institution_id: String, initial_products: Vec<crate::models::Products>) -> SandboxPublicTokenCreateRequest {
        SandboxPublicTokenCreateRequest {
            client_id: None,
            secret: None,
            institution_id,
            initial_products,
            options: None,
        }
    }
}


