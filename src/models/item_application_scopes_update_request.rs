/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// ItemApplicationScopesUpdateRequest : ItemApplicationScopesUpdateRequest defines the request schema for `/item/application/scopes/update`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemApplicationScopesUpdateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    #[serde(rename = "application_id")]
    pub application_id: String,
    #[serde(rename = "scopes")]
    pub scopes: Box<crate::models::Scopes>,
    /// When scopes are updated during enrollment, this field must be populated with the state sent to the partner in the OAuth Login URI. This field is required when the context is `ENROLLMENT`.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "context")]
    pub context: crate::models::ScopesContext,
}

impl ItemApplicationScopesUpdateRequest {
    /// ItemApplicationScopesUpdateRequest defines the request schema for `/item/application/scopes/update`
    pub fn new(
        access_token: String,
        application_id: String,
        scopes: crate::models::Scopes,
        context: crate::models::ScopesContext,
    ) -> ItemApplicationScopesUpdateRequest {
        ItemApplicationScopesUpdateRequest {
            client_id: None,
            secret: None,
            access_token,
            application_id,
            scopes: Box::new(scopes),
            state: None,
            context,
        }
    }
}
