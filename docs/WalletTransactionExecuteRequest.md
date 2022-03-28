# WalletTransactionExecuteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body. | [optional]
**secret** | Option<**String**> | Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body. | [optional]
**idempotency_key** | **String** | A random key provided by the client, per unique wallet transaction. Maximum of 128 characters.  The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. If a request to execute a wallet transaction fails due to a network connection error, then after a minimum delay of one minute, you can retry the request with the same idempotency key to guarantee that only a single wallet transaction is created. If the request was successfully processed, it will prevent any transaction that uses the same idempotency key, and was received within 24 hours of the first request, from being processed. | 
**wallet_id** | **String** | The ID of the e-wallet to debit from | 
**counterparty** | [**crate::models::WalletTransactionCounterparty**](WalletTransactionCounterparty.md) |  | 
**amount** | [**crate::models::WalletTransactionAmount**](WalletTransactionAmount.md) |  | 
**reference** | **String** | A reference for the transaction. This must be an alphanumeric string with at most 18 characters and must not contain any special characters or spaces. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


