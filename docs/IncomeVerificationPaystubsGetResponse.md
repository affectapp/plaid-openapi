# IncomeVerificationPaystubsGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**document_metadata** | Option<[**Vec<crate::models::DocumentMetadata>**](DocumentMetadata.md)> | Metadata for an income document. | [optional]
**paystubs** | [**Vec<crate::models::Paystub>**](Paystub.md) |  | 
**error** | Option<[**crate::models::PlaidError**](PlaidError.md)> |  | [optional]
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


