/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InstitutionsSearchRequestOptions : An optional object to filter `/institutions/search` results.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstitutionsSearchRequestOptions {
    /// Limit results to institutions with or without OAuth login flows.
    #[serde(rename = "oauth", skip_serializing_if = "Option::is_none")]
    pub oauth: Option<bool>,
    /// When true, return the institution's homepage URL, logo and primary brand color.
    #[serde(rename = "include_optional_metadata", skip_serializing_if = "Option::is_none")]
    pub include_optional_metadata: Option<bool>,
    /// When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    #[serde(rename = "include_auth_metadata", skip_serializing_if = "Option::is_none")]
    pub include_auth_metadata: Option<bool>,
    /// When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    #[serde(rename = "include_payment_initiation_metadata", skip_serializing_if = "Option::is_none")]
    pub include_payment_initiation_metadata: Option<bool>,
    #[serde(rename = "payment_initiation", skip_serializing_if = "Option::is_none")]
    pub payment_initiation: Option<crate::models::InstitutionsSearchPaymentInitiationOptions>,
}

impl InstitutionsSearchRequestOptions {
    /// An optional object to filter `/institutions/search` results.
    pub fn new() -> InstitutionsSearchRequestOptions {
        InstitutionsSearchRequestOptions {
            oauth: None,
            include_optional_metadata: None,
            include_auth_metadata: None,
            include_payment_initiation_metadata: None,
            payment_initiation: None,
        }
    }
}


