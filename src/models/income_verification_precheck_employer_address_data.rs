/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationPrecheckEmployerAddressData : Data about the components comprising an address.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckEmployerAddressData {
    /// The full city name
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The region or state. In API versions 2018-05-22 and earlier, this field is called `state`. Example: `\"NC\"`
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The full street address Example: `\"564 Main Street, APT 15\"`
    #[serde(rename = "street", skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// The postal code. In API versions 2018-05-22 and earlier, this field is called `zip`.
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The ISO 3166-1 alpha-2 country code
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl IncomeVerificationPrecheckEmployerAddressData {
    /// Data about the components comprising an address.
    pub fn new() -> IncomeVerificationPrecheckEmployerAddressData {
        IncomeVerificationPrecheckEmployerAddressData {
            city: None,
            region: None,
            street: None,
            postal_code: None,
            country: None,
        }
    }
}


