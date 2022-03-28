/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserCustomPassword : Custom test accounts are configured with a JSON configuration object formulated according to the schema below. All fields are optional. Sending an empty object as a configuration will result in an account configured with random balances and transaction history.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserCustomPassword {
    /// The version of the password schema to use, possible values are 1 or 2. The default value is 2. You should only specify 1 if you know it is necessary for your test suite.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// A seed, in the form of a string, that will be used to randomly generate account and transaction data, if this data is not specified using the `override_accounts` argument. If no seed is specified, the randomly generated data will be different each time.  Note that transactions data is generated relative to the Item's creation date. Different Items created on different dates with the same seed for transactions data will have different dates for the transactions. The number of days between each transaction and the Item creation will remain constant. For example, an Item created on December 15 might show a transaction on December 14. An Item created on December 20, using the same seed, would show that same transaction occurring on December 19.
    #[serde(rename = "seed")]
    pub seed: String,
    /// An array of account overrides to configure the accounts for the Item. By default, if no override is specified, transactions and account data will be randomly generated based on the account type and subtype, and other products will have fixed or empty data.
    #[serde(rename = "override_accounts")]
    pub override_accounts: Vec<crate::models::OverrideAccounts>,
    #[serde(rename = "mfa")]
    pub mfa: crate::models::Mfa,
    /// You may trigger a reCAPTCHA in Plaid Link in the Sandbox environment by using the recaptcha field. Possible values are `good` or `bad`. A value of `good` will result in successful Item creation and `bad` will result in a `RECAPTCHA_BAD` error to simulate a failed reCAPTCHA. Both values require the reCAPTCHA to be manually solved within Plaid Link.
    #[serde(rename = "recaptcha")]
    pub recaptcha: String,
    /// An error code to force on Item creation. Possible values are:  `\"INSTITUTION_NOT_RESPONDING\"` `\"INSTITUTION_NO_LONGER_SUPPORTED\"` `\"INVALID_CREDENTIALS\"` `\"INVALID_MFA\"` `\"ITEM_LOCKED\"` `\"ITEM_LOGIN_REQUIRED\"` `\"ITEM_NOT_SUPPORTED\"` `\"INVALID_LINK_TOKEN\"` `\"MFA_NOT_SUPPORTED\"` `\"NO_ACCOUNTS\"` `\"PLAID_ERROR\"` `\"PRODUCTS_NOT_SUPPORTED\"` `\"USER_SETUP_REQUIRED\"`
    #[serde(rename = "force_error")]
    pub force_error: String,
}

impl UserCustomPassword {
    /// Custom test accounts are configured with a JSON configuration object formulated according to the schema below. All fields are optional. Sending an empty object as a configuration will result in an account configured with random balances and transaction history.
    pub fn new(seed: String, override_accounts: Vec<crate::models::OverrideAccounts>, mfa: crate::models::Mfa, recaptcha: String, force_error: String) -> UserCustomPassword {
        UserCustomPassword {
            version: None,
            seed,
            override_accounts,
            mfa,
            recaptcha,
            force_error,
        }
    }
}


