# Transfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Plaid’s unique identifier for a transfer. | 
**ach_class** | [**crate::models::AchClass**](ACHClass.md) |  | 
**account_id** | **String** | The account ID that should be credited/debited for this transfer. | 
**_type** | [**crate::models::TransferType**](TransferType.md) |  | 
**user** | [**crate::models::TransferUserInResponse**](TransferUserInResponse.md) |  | 
**amount** | **String** | The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). | 
**description** | **String** | The description of the transfer. | 
**created** | **String** | The datetime when this transfer was created. This will be of the form `2006-01-02T15:04:05Z` | 
**status** | [**crate::models::TransferStatus**](TransferStatus.md) |  | 
**sweep_status** | Option<[**crate::models::TransferSweepStatus**](TransferSweepStatus.md)> |  | [optional]
**network** | [**crate::models::TransferNetwork**](TransferNetwork.md) |  | 
**cancellable** | **bool** | When `true`, you can still cancel this transfer. | 
**failure_reason** | Option<[**crate::models::TransferFailure**](TransferFailure.md)> |  | 
**metadata** | Option<**::std::collections::HashMap<String, String>**> | The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: - The JSON values must be Strings (no nested JSON objects allowed) - Only ASCII characters may be used - Maximum of 50 key/value pairs - Maximum key length of 40 characters - Maximum value length of 500 characters  | 
**origination_account_id** | **String** | Plaid’s unique identifier for the origination account that was used for this transfer. | 
**guarantee_decision** | Option<[**crate::models::TransferAuthorizationGuaranteeDecision**](TransferAuthorizationGuaranteeDecision.md)> |  | 
**guarantee_decision_rationale** | Option<[**crate::models::TransferAuthorizationGuaranteeDecisionRationale**](TransferAuthorizationGuaranteeDecisionRationale.md)> |  | 
**iso_currency_code** | **String** | The currency of the transfer amount, e.g. \"USD\" | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


