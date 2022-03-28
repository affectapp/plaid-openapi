# LinkTokenCreateRequestIncomeVerification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**income_verification_id** | Option<**String**> | The `income_verification_id` of the verification instance, as provided by `/income/verification/create`. | [optional]
**asset_report_id** | Option<**String**> | The `asset_report_id` of an asset report associated with the user, as provided by `/asset_report/create`. Providing an `asset_report_id` is optional and can be used to verify the user through a streamlined flow. If provided, the bank linking flow will be skipped. | [optional]
**precheck_id** | Option<**String**> | The ID of a precheck created with `/income/verification/precheck`. Will be used to improve conversion of the income verification flow by streamlining the Link interface presented to the end user. | [optional]
**access_tokens** | Option<**Vec<String>**> | An array of access tokens corresponding to the Items that will be cross-referenced with the product data. If the `transactions` product was not initialized for the Items during link, it will be initialized after this Link session. | [optional]
**income_source_types** | Option<[**Vec<crate::models::IncomeVerificationSourceType>**](IncomeVerificationSourceType.md)> | The types of source income data that users will be permitted to share. Options include `bank` and `payroll`. Currently you can only specify one of these options. | [optional]
**bank_income** | Option<[**crate::models::LinkTokenCreateRequestIncomeVerificationBankIncome**](LinkTokenCreateRequestIncomeVerificationBankIncome.md)> |  | [optional]
**payroll_income** | Option<[**crate::models::LinkTokenCreateRequestIncomeVerificationPayrollIncome**](LinkTokenCreateRequestIncomeVerificationPayrollIncome.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


