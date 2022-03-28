/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// InstitutionsSearchRequest : InstitutionsSearchRequest defines the request schema for `/institutions/search`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstitutionsSearchRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The search query. Institutions with names matching the query are returned
    #[serde(rename = "query")]
    pub query: String,
    /// Filter the Institutions based on whether they support all products listed in `products`. Provide `null` to get institutions regardless of supported products. Note that when `auth` is specified as a product, if you are enabled for Instant Match or Automated Micro-deposits, institutions that support those products will be returned even if `auth` is not present in their product array.
    #[serde(rename = "products")]
    pub products: Option<Vec<crate::models::Products>>,
    /// Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard. In API versions 2019-05-29 and earlier, the `country_codes` parameter is an optional parameter within the `options` object and will default to `[US]` if it is not supplied.
    #[serde(rename = "country_codes")]
    pub country_codes: Vec<crate::models::CountryCode>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::InstitutionsSearchRequestOptions>>,
}

impl InstitutionsSearchRequest {
    /// InstitutionsSearchRequest defines the request schema for `/institutions/search`
    pub fn new(
        query: String,
        products: Option<Vec<crate::models::Products>>,
        country_codes: Vec<crate::models::CountryCode>,
    ) -> InstitutionsSearchRequest {
        InstitutionsSearchRequest {
            client_id: None,
            secret: None,
            query,
            products,
            country_codes,
            options: None,
        }
    }
}
