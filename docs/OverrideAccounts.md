# OverrideAccounts

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | [**crate::models::OverrideAccountType**](OverrideAccountType.md) |  | 
**subtype** | Option<[**crate::models::AccountSubtype**](AccountSubtype.md)> |  | 
**starting_balance** | **f64** | If provided, the account will start with this amount as the current balance.  | 
**force_available_balance** | **f64** | If provided, the account will always have this amount as its  available balance, regardless of current balance or changes in transactions over time. | 
**currency** | **String** | ISO-4217 currency code. If provided, the account will be denominated in the given currency. Transactions will also be in this currency by default. | 
**meta** | [**crate::models::Meta**](Meta.md) |  | 
**numbers** | [**crate::models::Numbers**](Numbers.md) |  | 
**transactions** | [**Vec<crate::models::TransactionOverride>**](TransactionOverride.md) | Specify the list of transactions on the account. | 
**holdings** | Option<[**crate::models::HoldingsOverride**](HoldingsOverride.md)> |  | [optional]
**investment_transactions** | Option<[**crate::models::InvestmentsTransactionsOverride**](Investments_TransactionsOverride.md)> |  | [optional]
**identity** | [**crate::models::OwnerOverride**](OwnerOverride.md) |  | 
**liability** | [**crate::models::LiabilityOverride**](LiabilityOverride.md) |  | 
**inflow_model** | [**crate::models::InflowModel**](InflowModel.md) |  | 
**income** | Option<[**crate::models::IncomeOverride**](IncomeOverride.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


