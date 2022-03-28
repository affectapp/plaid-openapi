# PaymentInitiationMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**supports_international_payments** | **bool** | Indicates whether the institution supports payments from a different country. | 
**maximum_payment_amount** | **::std::collections::HashMap<String, String>** | A mapping of currency to maximum payment amount (denominated in the smallest unit of currency) supported by the institution.  Example: `{\"GBP\": \"10000\"}`  | 
**supports_refund_details** | **bool** | Indicates whether the institution supports returning refund details when initiating a payment. | 
**standing_order_metadata** | Option<[**crate::models::PaymentInitiationStandingOrderMetadata**](PaymentInitiationStandingOrderMetadata.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


