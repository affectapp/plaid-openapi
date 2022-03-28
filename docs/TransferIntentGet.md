# TransferIntentGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Plaid's unique identifier for a transfer intent object. | 
**created** | **String** | The datetime the transfer was created. This will be of the form `2006-01-02T15:04:05Z`. | 
**status** | [**crate::models::TransferIntentStatus**](TransferIntentStatus.md) |  | 
**transfer_id** | Option<**String**> | Plaid's unique identifier for the transfer created through the UI. Returned only if the transfer was successfully created. Null value otherwise. | 
**failure_reason** | Option<[**crate::models::TransferIntentGetFailureReason**](TransferIntentGetFailureReason.md)> |  | 
**authorization_decision** | Option<[**crate::models::TransferIntentAuthorizationDecision**](TransferIntentAuthorizationDecision.md)> |  | 
**authorization_decision_rationale** | Option<[**crate::models::TransferAuthorizationDecisionRationale**](TransferAuthorizationDecisionRationale.md)> |  | 
**account_id** | Option<**String**> | The Plaid `account_id` for the account that will be debited or credited. Returned only if `account_id` was set on intent creation. | [optional]
**origination_account_id** | **String** | Plaid’s unique identifier for the origination account used for the transfer. | 
**amount** | **String** | The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**mode** | [**crate::models::TransferIntentCreateMode**](TransferIntentCreateMode.md) |  | 
**ach_class** | [**crate::models::AchClass**](ACHClass.md) |  | 
**user** | [**crate::models::TransferUserInResponse**](TransferUserInResponse.md) |  | 
**description** | **String** | A description for the underlying transfer. Maximum of 8 characters. | 
**metadata** | Option<**::std::collections::HashMap<String, String>**> | The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: - The JSON values must be Strings (no nested JSON objects allowed) - Only ASCII characters may be used - Maximum of 50 key/value pairs - Maximum key length of 40 characters - Maximum value length of 500 characters  | [optional]
**iso_currency_code** | **String** | The currency of the transfer amount, e.g. \"USD\" | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


