/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemImportRequest : ItemImportRequest defines the request schema for `/item/import`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemImportRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Array of product strings
    #[serde(rename = "products")]
    pub products: Vec<crate::models::Products>,
    #[serde(rename = "user_auth")]
    pub user_auth: Box<crate::models::ItemImportRequestUserAuth>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::ItemImportRequestOptions>>,
}

impl ItemImportRequest {
    /// ItemImportRequest defines the request schema for `/item/import`
    pub fn new(products: Vec<crate::models::Products>, user_auth: crate::models::ItemImportRequestUserAuth) -> ItemImportRequest {
        ItemImportRequest {
            client_id: None,
            secret: None,
            products,
            user_auth: Box::new(user_auth),
            options: None,
        }
    }
}


