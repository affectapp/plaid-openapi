# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_id** | **String** | The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive. | 
**institution_id** | Option<**String**> | The Plaid Institution ID associated with the Item. Field is `null` for Items created via Same Day Micro-deposits. | [optional]
**webhook** | Option<**String**> | The URL registered to receive webhooks for the Item. | 
**error** | Option<[**crate::models::Error**](Error.md)> |  | 
**available_products** | [**Vec<crate::models::Products>**](Products.md) | A list of products available for the Item that have not yet been accessed. | 
**billed_products** | [**Vec<crate::models::Products>**](Products.md) | A list of products that have been billed for the Item. Note - `billed_products` is populated in all environments but only requests in Production are billed.  | 
**products** | Option<[**Vec<crate::models::Products>**](Products.md)> | A list of authorized products for the Item.  | [optional]
**consented_products** | Option<[**Vec<crate::models::Products>**](Products.md)> | Beta: A list of products that have gone through consent collection for the Item. Only present for those enabled in the beta.  | [optional]
**consent_expiration_time** | Option<**String**> | The RFC 3339 timestamp after which the consent provided by the end user will expire. Upon consent expiration, the item will enter the `ITEM_LOGIN_REQUIRED` error state. To circumvent the `ITEM_LOGIN_REQUIRED` error and maintain continuous consent, the end user can reauthenticate via Link’s update mode in advance of the consent expiration time.  Note - This is only relevant for certain OAuth-based institutions. For all other institutions, this field will be null.  | 
**update_type** | **String** | Indicates whether an Item requires user interaction to be updated, which can be the case for Items with some forms of two-factor authentication.  `background` - Item can be updated in the background  `user_present_required` - Item requires user interaction to be updated | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


