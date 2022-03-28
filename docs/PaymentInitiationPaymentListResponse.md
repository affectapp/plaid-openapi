# PaymentInitiationPaymentListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payments** | [**Vec<crate::models::PaymentInitiationPayment>**](PaymentInitiationPayment.md) | An array of payments that have been created, associated with the given `client_id`. | 
**next_cursor** | Option<**String**> | The value that, when used as the optional `cursor` parameter to `/payment_initiation/payment/list`, will return the next unreturned payment as its first payment. | 
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


