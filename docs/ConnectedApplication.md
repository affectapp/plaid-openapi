# ConnectedApplication

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_id** | **String** | This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect. | 
**name** | **String** | The name of the application | 
**logo** | Option<**String**> | A URL that links to the application logo image (will be deprecated in the future, please use logo_url). | 
**logo_url** | Option<**String**> | A URL that links to the application logo image. | 
**application_url** | Option<**String**> | The URL for the application's website | 
**reason_for_access** | Option<**String**> | A string provided by the connected app stating why they use their respective enabled products. | 
**created_at** | [**String**](string.md) | The date this application was linked in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC. | 
**join_date** | [**String**](string.md) | The date this application was granted production access at Plaid in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC. | 
**product_data_types** | **Vec<String>** | (Deprecated) A list of enums representing the data collected and products enabled for this connected application. | 
**scopes** | Option<[**crate::models::ScopesNullable**](ScopesNullable.md)> |  | [optional]
**requested_scopes** | Option<[**crate::models::RequestedScopes**](RequestedScopes.md)> |  | [optional]
**status** | [**crate::models::ConnectedApplicationStatus**](ConnectedApplicationStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


