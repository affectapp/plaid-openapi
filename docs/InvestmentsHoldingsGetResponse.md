# InvestmentsHoldingsGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accounts** | [**Vec<crate::models::AccountBase>**](AccountBase.md) | The accounts associated with the Item | 
**holdings** | [**Vec<crate::models::Holding>**](Holding.md) | The holdings belonging to investment accounts associated with the Item. Details of the securities in the holdings are provided in the `securities` field.  | 
**securities** | [**Vec<crate::models::Security>**](Security.md) | Objects describing the securities held in the accounts associated with the Item.  | 
**item** | [**crate::models::Item**](Item.md) |  | 
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


