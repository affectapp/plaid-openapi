# ItemImportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**products** | [**Vec<crate::models::Products>**](Products.md) | Array of product strings | 
**user_auth** | [**crate::models::ItemImportRequestUserAuth**](ItemImportRequestUserAuth.md) |  | 
**options** | Option<[**crate::models::ItemImportRequestOptions**](ItemImportRequestOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


