/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InstitutionsGetByIdRequestOptions : Specifies optional parameters for `/institutions/get_by_id`. If provided, must not be `null`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstitutionsGetByIdRequestOptions {
    /// When `true`, return an institution's logo, brand color, and URL. When available, the bank's logo is returned as a base64 encoded 152x152 PNG, the brand color is in hexadecimal format. The default value is `false`.  Note that Plaid does not own any of the logos shared by the API and that by accessing or using these logos, you agree that you are doing so at your own risk and will, if necessary, obtain all required permissions from the appropriate rights holders and adhere to any applicable usage guidelines. Plaid disclaims all express or implied warranties with respect to the logos.
    #[serde(rename = "include_optional_metadata", skip_serializing_if = "Option::is_none")]
    pub include_optional_metadata: Option<bool>,
    /// If `true`, the response will include status information about the institution. Default value is `false`.
    #[serde(rename = "include_status", skip_serializing_if = "Option::is_none")]
    pub include_status: Option<bool>,
    /// When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    #[serde(rename = "include_auth_metadata", skip_serializing_if = "Option::is_none")]
    pub include_auth_metadata: Option<bool>,
    /// When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    #[serde(rename = "include_payment_initiation_metadata", skip_serializing_if = "Option::is_none")]
    pub include_payment_initiation_metadata: Option<bool>,
}

impl InstitutionsGetByIdRequestOptions {
    /// Specifies optional parameters for `/institutions/get_by_id`. If provided, must not be `null`.
    pub fn new() -> InstitutionsGetByIdRequestOptions {
        InstitutionsGetByIdRequestOptions {
            include_optional_metadata: None,
            include_status: None,
            include_auth_metadata: None,
            include_payment_initiation_metadata: None,
        }
    }
}


