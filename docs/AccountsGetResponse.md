# AccountsGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accounts** | [**Vec<crate::models::AccountBase>**](AccountBase.md) | An array of financial institution accounts associated with the Item. If `/accounts/balance/get` was called, each account will include real-time balance information. | 
**item** | [**crate::models::Item**](Item.md) |  | 
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


