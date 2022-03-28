# Numbers

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | Option<**String**> | Will be used for the account number. | [optional]
**ach_routing** | Option<**String**> | Must be a valid ACH routing number. | [optional]
**ach_wire_routing** | Option<**String**> | Must be a valid wire transfer routing number. | [optional]
**eft_institution** | Option<**String**> | EFT institution number. Must be specified alongside `eft_branch`. | [optional]
**eft_branch** | Option<**String**> | EFT branch number. Must be specified alongside `eft_institution`. | [optional]
**international_bic** | Option<**String**> | Bank identifier code (BIC). Must be specified alongside `international_iban`. | [optional]
**international_iban** | Option<**String**> | International bank account number (IBAN). If no account number is specified via `account`, will also be used as the account number by default. Must be specified alongside `international_bic`. | [optional]
**bacs_sort_code** | Option<**String**> | BACS sort code | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


