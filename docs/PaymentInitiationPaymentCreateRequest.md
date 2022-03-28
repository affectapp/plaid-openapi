# PaymentInitiationPaymentCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**recipient_id** | **String** | The ID of the recipient the payment is for. | 
**reference** | **String** | A reference for the payment. This must be an alphanumeric string with at most 18 characters and must not contain any special characters (since not all institutions support them). | 
**amount** | [**crate::models::PaymentAmount**](PaymentAmount.md) |  | 
**schedule** | Option<[**crate::models::ExternalPaymentScheduleRequest**](ExternalPaymentScheduleRequest.md)> |  | [optional]
**options** | Option<[**crate::models::ExternalPaymentOptions**](ExternalPaymentOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


