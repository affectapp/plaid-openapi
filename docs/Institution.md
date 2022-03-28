# Institution

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**institution_id** | **String** | Unique identifier for the institution | 
**name** | **String** | The official name of the institution | 
**products** | [**Vec<crate::models::Products>**](Products.md) | A list of the Plaid products supported by the institution. Note that only institutions that support Instant Auth will return `auth` in the product array; institutions that do not list `auth` may still support other Auth methods such as Instant Match or Automated Micro-deposit Verification. To identify institutions that support those methods, use the `auth_metadata` object. For more details, see [Full Auth coverage](https://plaid.com/docs/auth/coverage/). | 
**country_codes** | [**Vec<crate::models::CountryCode>**](CountryCode.md) | A list of the country codes supported by the institution. | 
**url** | Option<**String**> | The URL for the institution's website | [optional]
**primary_color** | Option<**String**> | Hexadecimal representation of the primary color used by the institution | [optional]
**logo** | Option<**String**> | Base64 encoded representation of the institution's logo | [optional]
**routing_numbers** | **Vec<String>** | A partial list of routing numbers associated with the institution. This list is provided for the purpose of looking up institutions by routing number. It is not comprehensive and should never be used as a complete list of routing numbers for an institution. | 
**oauth** | **bool** | Indicates that the institution has an OAuth login flow. | 
**status** | Option<[**crate::models::InstitutionStatus**](InstitutionStatus.md)> |  | [optional]
**payment_initiation_metadata** | Option<[**crate::models::PaymentInitiationMetadata**](PaymentInitiationMetadata.md)> |  | [optional]
**auth_metadata** | Option<[**crate::models::AuthMetadata**](AuthMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


