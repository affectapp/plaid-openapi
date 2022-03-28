# PaymentStatusUpdateWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | **String** | `PAYMENT_INITIATION` | 
**webhook_code** | **String** | `PAYMENT_STATUS_UPDATE` | 
**payment_id** | **String** | The `payment_id` for the payment being updated | 
**new_payment_status** | [**crate::models::PaymentInitiationPaymentStatus**](PaymentInitiationPaymentStatus.md) |  | 
**old_payment_status** | [**crate::models::PaymentInitiationPaymentStatus**](PaymentInitiationPaymentStatus.md) |  | 
**original_reference** | Option<**String**> | The original value of the reference when creating the payment. | 
**adjusted_reference** | Option<**String**> | The value of the reference sent to the bank after adjustment to pass bank validation rules. | [optional]
**original_start_date** | Option<[**String**](string.md)> | The original value of the `start_date` provided during the creation of a standing order. If the payment is not a standing order, this field will be `null`. | 
**adjusted_start_date** | Option<[**String**](string.md)> | The start date sent to the bank after adjusting for holidays or weekends.  Will be provided in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). If the start date did not require adjustment, or if the payment is not a standing order, this field will be `null`. | 
**timestamp** | **String** | The timestamp of the update, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `\"2017-09-14T14:42:19.350Z\"` | 
**error** | Option<[**crate::models::PlaidError**](PlaidError.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


