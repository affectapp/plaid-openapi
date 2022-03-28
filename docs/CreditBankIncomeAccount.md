# CreditBankIncomeAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | Plaid's unique identifier for the account. | [optional]
**mask** | Option<**String**> | The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user. | [optional]
**name** | Option<**String**> | The name of the bank account. | [optional]
**official_name** | Option<**String**> | The official name of the bank account. | [optional]
**subtype** | Option<[**crate::models::DepositoryAccountSubtype**](DepositoryAccountSubtype.md)> |  | [optional]
**_type** | Option<[**crate::models::CreditBankIncomeAccountType**](CreditBankIncomeAccountType.md)> |  | [optional]
**owners** | Option<[**Vec<crate::models::Owner>**](Owner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


