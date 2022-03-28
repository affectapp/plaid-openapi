/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsRulesCreateRequest : TransactionsRulesCreateRequest defines the request schema for `/transactions/rules/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionsRulesCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret")]
    pub secret: String,
    /// Personal finance detailed category.  See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories. 
    #[serde(rename = "personal_finance_category")]
    pub personal_finance_category: String,
    #[serde(rename = "rule_details")]
    pub rule_details: Box<crate::models::TransactionsRuleDetails>,
}

impl TransactionsRulesCreateRequest {
    /// TransactionsRulesCreateRequest defines the request schema for `/transactions/rules/create`
    pub fn new(client_id: String, access_token: String, secret: String, personal_finance_category: String, rule_details: crate::models::TransactionsRuleDetails) -> TransactionsRulesCreateRequest {
        TransactionsRulesCreateRequest {
            client_id,
            access_token,
            secret,
            personal_finance_category,
            rule_details: Box::new(rule_details),
        }
    }
}


