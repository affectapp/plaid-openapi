# InvestmentsTransactionsGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item** | [**crate::models::Item**](Item.md) |  | 
**accounts** | [**Vec<crate::models::AccountBase>**](AccountBase.md) | The accounts for which transaction history is being fetched. | 
**securities** | [**Vec<crate::models::Security>**](Security.md) | All securities for which there is a corresponding transaction being fetched. | 
**investment_transactions** | [**Vec<crate::models::InvestmentTransaction>**](InvestmentTransaction.md) | The transactions being fetched | 
**total_investment_transactions** | **i32** | The total number of transactions available within the date range specified. If `total_investment_transactions` is larger than the size of the `transactions` array, more transactions are available and can be fetched via manipulating the `offset` parameter.' | 
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


