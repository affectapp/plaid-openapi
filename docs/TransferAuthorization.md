# TransferAuthorization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Plaid’s unique identifier for a transfer authorization. | 
**created** | **String** | The datetime representing when the authorization was created, in the format `2006-01-02T15:04:05Z`. | 
**decision** | [**crate::models::TransferAuthorizationDecision**](TransferAuthorizationDecision.md) |  | 
**decision_rationale** | Option<[**crate::models::TransferAuthorizationDecisionRationale**](TransferAuthorizationDecisionRationale.md)> |  | 
**guarantee_decision** | Option<[**crate::models::TransferAuthorizationGuaranteeDecision**](TransferAuthorizationGuaranteeDecision.md)> |  | 
**guarantee_decision_rationale** | Option<[**crate::models::TransferAuthorizationGuaranteeDecisionRationale**](TransferAuthorizationGuaranteeDecisionRationale.md)> |  | 
**proposed_transfer** | [**crate::models::TransferAuthorizationProposedTransfer**](TransferAuthorizationProposedTransfer.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


