# BankTransferFailure

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ach_return_code** | Option<**String**> | The ACH return code, e.g. `R01`.  A return code will be provided if and only if the transfer status is `reversed`. For a full listing of ACH return codes, see [Bank Transfers errors](https://plaid.com/docs/errors/bank-transfers/#ach-return-codes). | [optional]
**description** | Option<**String**> | A human-readable description of the reason for the failure or reversal. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


