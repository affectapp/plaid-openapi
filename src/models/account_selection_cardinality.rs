/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccountSelectionCardinality : The application requires that accounts be limited to a specific cardinality. `MULTI_SELECT`: indicates that the user should be allowed to pick multiple accounts. `SINGLE_SELECT`: indicates that the user should be allowed to pick only a single account. `ALL`: indicates that the user must share all of their accounts and should not be given the opportunity to de-select

/// The application requires that accounts be limited to a specific cardinality. `MULTI_SELECT`: indicates that the user should be allowed to pick multiple accounts. `SINGLE_SELECT`: indicates that the user should be allowed to pick only a single account. `ALL`: indicates that the user must share all of their accounts and should not be given the opportunity to de-select
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountSelectionCardinality {
    #[serde(rename = "SINGLE_SELECT")]
    SINGLESELECT,
    #[serde(rename = "MULTI_SELECT")]
    MULTISELECT,
    #[serde(rename = "ALL")]
    ALL,

}

impl ToString for AccountSelectionCardinality {
    fn to_string(&self) -> String {
        match self {
            Self::SINGLESELECT => String::from("SINGLE_SELECT"),
            Self::MULTISELECT => String::from("MULTI_SELECT"),
            Self::ALL => String::from("ALL"),
        }
    }
}

impl Default for AccountSelectionCardinality {
    fn default() -> AccountSelectionCardinality {
        Self::SINGLESELECT
    }
}




