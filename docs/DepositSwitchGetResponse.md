# DepositSwitchGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deposit_switch_id** | **String** | The ID of the deposit switch. | 
**target_account_id** | Option<**String**> | The ID of the bank account the direct deposit was switched to. | 
**target_item_id** | Option<**String**> | The ID of the Item the direct deposit was switched to. | 
**state** | **String** |  The state, or status, of the deposit switch.  - `initialized` – The deposit switch has been initialized with the user entering the information required to submit the deposit switch request.  - `processing` – The deposit switch request has been submitted and is being processed.  - `completed` – The user's employer has fulfilled the deposit switch request.  - `error` – There was an error processing the deposit switch request. | 
**switch_method** | Option<**String**> | The method used to make the deposit switch.  - `instant` – User instantly switched their direct deposit to a new or existing bank account by connecting their payroll or employer account.  - `mail` – User requested that Plaid contact their employer by mail to make the direct deposit switch.  - `pdf` – User generated a PDF or email to be sent to their employer with the information necessary to make the deposit switch.' | [optional]
**account_has_multiple_allocations** | Option<**bool**> | When `true`, user’s direct deposit goes to multiple banks. When false, user’s direct deposit only goes to the target account. Always `null` if the deposit switch has not been completed. | 
**is_allocated_remainder** | Option<**bool**> | When `true`, the target account is allocated the remainder of direct deposit after all other allocations have been deducted. When `false`, user’s direct deposit is allocated as a percent or amount. Always `null` if the deposit switch has not been completed. | 
**percent_allocated** | Option<**f64**> | The percentage of direct deposit allocated to the target account. Always `null` if the target account is not allocated a percentage or if the deposit switch has not been completed or if `is_allocated_remainder` is true. | 
**amount_allocated** | Option<**f64**> | The dollar amount of direct deposit allocated to the target account. Always `null` if the target account is not allocated an amount or if the deposit switch has not been completed. | 
**employer_name** | Option<**String**> | The name of the employer selected by the user. If the user did not select an employer, the value returned is `null`. | [optional]
**employer_id** | Option<**String**> | The ID of the employer selected by the user. If the user did not select an employer, the value returned is `null`. | [optional]
**institution_name** | Option<**String**> | The name of the institution selected by the user. If the user did not select an institution, the value returned is `null`. | [optional]
**institution_id** | Option<**String**> | The ID of the institution selected by the user. If the user did not select an institution, the value returned is `null`. | [optional]
**date_created** | [**String**](string.md) | [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date the deposit switch was created.  | 
**date_completed** | Option<[**String**](string.md)> | [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date the deposit switch was completed. Always `null` if the deposit switch has not been completed.  | 
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


