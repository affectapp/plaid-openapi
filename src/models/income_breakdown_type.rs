/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// IncomeBreakdownType : The type of income. Possible values include:   `\"regular\"`: regular income   `\"overtime\"`: overtime income   `\"bonus\"`: bonus income

/// The type of income. Possible values include:   `\"regular\"`: regular income   `\"overtime\"`: overtime income   `\"bonus\"`: bonus income
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncomeBreakdownType {
    #[serde(rename = "bonus")]
    Bonus,
    #[serde(rename = "overtime")]
    Overtime,
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "null")]
    Null,
}

impl ToString for IncomeBreakdownType {
    fn to_string(&self) -> String {
        match self {
            Self::Bonus => String::from("bonus"),
            Self::Overtime => String::from("overtime"),
            Self::Regular => String::from("regular"),
            Self::Null => String::from("null"),
        }
    }
}

impl Default for IncomeBreakdownType {
    fn default() -> IncomeBreakdownType {
        Self::Bonus
    }
}
