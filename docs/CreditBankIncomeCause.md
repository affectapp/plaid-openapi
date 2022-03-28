# CreditBankIncomeCause

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error_type** | Option<[**crate::models::CreditBankIncomeErrorType**](CreditBankIncomeErrorType.md)> |  | [optional]
**error_code** | Option<**String**> | We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. Error fields will be `null` if no error has occurred. | [optional]
**error_message** | Option<**String**> | A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use. | [optional]
**display_message** | Option<**String**> | A user-friendly representation of the error code. null if the error is not related to user action. This may change over time and is not safe for programmatic use. | [optional]
**item_id** | Option<**String**> | The `item_id` of the Item associated with this warning. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


