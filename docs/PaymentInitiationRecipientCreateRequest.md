# PaymentInitiationRecipientCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**name** | **String** | The name of the recipient | 
**iban** | Option<**String**> | The International Bank Account Number (IBAN) for the recipient. If BACS data is not provided, an IBAN is required. | [optional]
**bacs** | Option<[**crate::models::RecipientBacsNullable**](RecipientBACSNullable.md)> |  | [optional]
**address** | Option<[**crate::models::PaymentInitiationAddress**](PaymentInitiationAddress.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


