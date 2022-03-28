# PaymentInitiationConsent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**consent_id** | **String** | The consent ID. | 
**status** | [**crate::models::PaymentInitiationConsentStatus**](PaymentInitiationConsentStatus.md) |  | 
**created_at** | **String** | Consent creation timestamp, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. | 
**recipient_id** | **String** | The ID of the recipient the payment consent is for. | 
**reference** | **String** | A reference for the payment consent. | 
**constraints** | [**crate::models::PaymentInitiationConsentConstraints**](PaymentInitiationConsentConstraints.md) |  | 
**scopes** | [**Vec<crate::models::PaymentInitiationConsentScope>**](PaymentInitiationConsentScope.md) | An array of payment consent scopes. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


