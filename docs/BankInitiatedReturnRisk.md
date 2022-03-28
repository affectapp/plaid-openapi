# BankInitiatedReturnRisk

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**score** | **i32** | A score from 0-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood. | 
**risk_tier** | **i32** | In the `bank_initiated_return_risk` object, there are eight risk tiers corresponding to the scores:   1: Predicted bank-initiated return incidence rate between 0.0% - 0.5%   2: Predicted bank-initiated return incidence rate between 0.5% - 1.5%   3: Predicted bank-initiated return incidence rate between 1.5% - 3%   4: Predicted bank-initiated return incidence rate between 3% - 5%   5: Predicted bank-initiated return incidence rate between 5% - 10%   6: Predicted bank-initiated return incidence rate between 10% - 15%   7: Predicted bank-initiated return incidence rate between 15% and 50%   8: Predicted bank-initiated return incidence rate greater than 50%  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


