# LinkTokenGetMetadataResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**initial_products** | [**Vec<crate::models::Products>**](Products.md) | The `products` specified in the `/link/token/create` call. | 
**webhook** | Option<**String**> | The `webhook` specified in the `/link/token/create` call. | 
**country_codes** | [**Vec<crate::models::CountryCode>**](CountryCode.md) | The `country_codes` specified in the `/link/token/create` call. | 
**language** | Option<**String**> | The `language` specified in the `/link/token/create` call. | 
**account_filters** | Option<[**crate::models::AccountFiltersResponse**](AccountFiltersResponse.md)> |  | [optional]
**redirect_uri** | Option<**String**> | The `redirect_uri` specified in the `/link/token/create` call. | 
**client_name** | Option<**String**> | The `client_name` specified in the `/link/token/create` call. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


