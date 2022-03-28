/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CategoriesGetResponse : CategoriesGetResponse defines the response schema for `/categories/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CategoriesGetResponse {
    /// An array of all of the transaction categories used by Plaid.
    #[serde(rename = "categories")]
    pub categories: Vec<crate::models::Category>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl CategoriesGetResponse {
    /// CategoriesGetResponse defines the response schema for `/categories/get`
    pub fn new(categories: Vec<crate::models::Category>, request_id: String) -> CategoriesGetResponse {
        CategoriesGetResponse {
            categories,
            request_id,
        }
    }
}

