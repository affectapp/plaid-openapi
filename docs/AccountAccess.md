# AccountAccess

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**unique_id** | **String** | The unique account identifier for this account. This value must match that returned by the data access API for this account. | 
**authorized** | Option<**bool**> | Allow the application to see this account (and associated details, including balance) in the list of accounts  If unset, defaults to `true`. | [optional][default to true]
**account_product_access** | Option<[**crate::models::AccountProductAccessNullable**](AccountProductAccessNullable.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


