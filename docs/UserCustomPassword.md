# UserCustomPassword

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | Option<**String**> | The version of the password schema to use, possible values are 1 or 2. The default value is 2. You should only specify 1 if you know it is necessary for your test suite. | [optional]
**seed** | **String** | A seed, in the form of a string, that will be used to randomly generate account and transaction data, if this data is not specified using the `override_accounts` argument. If no seed is specified, the randomly generated data will be different each time.  Note that transactions data is generated relative to the Item's creation date. Different Items created on different dates with the same seed for transactions data will have different dates for the transactions. The number of days between each transaction and the Item creation will remain constant. For example, an Item created on December 15 might show a transaction on December 14. An Item created on December 20, using the same seed, would show that same transaction occurring on December 19. | 
**override_accounts** | [**Vec<crate::models::OverrideAccounts>**](OverrideAccounts.md) | An array of account overrides to configure the accounts for the Item. By default, if no override is specified, transactions and account data will be randomly generated based on the account type and subtype, and other products will have fixed or empty data. | 
**mfa** | [**crate::models::Mfa**](MFA.md) |  | 
**recaptcha** | **String** | You may trigger a reCAPTCHA in Plaid Link in the Sandbox environment by using the recaptcha field. Possible values are `good` or `bad`. A value of `good` will result in successful Item creation and `bad` will result in a `RECAPTCHA_BAD` error to simulate a failed reCAPTCHA. Both values require the reCAPTCHA to be manually solved within Plaid Link. | 
**force_error** | **String** | An error code to force on Item creation. Possible values are:  `\"INSTITUTION_NOT_RESPONDING\"` `\"INSTITUTION_NO_LONGER_SUPPORTED\"` `\"INVALID_CREDENTIALS\"` `\"INVALID_MFA\"` `\"ITEM_LOCKED\"` `\"ITEM_LOGIN_REQUIRED\"` `\"ITEM_NOT_SUPPORTED\"` `\"INVALID_LINK_TOKEN\"` `\"MFA_NOT_SUPPORTED\"` `\"NO_ACCOUNTS\"` `\"PLAID_ERROR\"` `\"PRODUCTS_NOT_SUPPORTED\"` `\"USER_SETUP_REQUIRED\"` | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


