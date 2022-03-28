# BankTransferUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**legal_name** | **String** | The account holder’s full legal name. If the transfer `ach_class` is `ccd`, this should be the business name of the account holder. | 
**email_address** | Option<**String**> | The account holder’s email. | [optional]
**routing_number** | Option<**String**> | The account holder's routing number. This field is only used in response data. Do not provide this field when making requests. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


