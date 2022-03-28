# StandaloneAccountType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**depository** | **String** | An account type holding cash, in which funds are deposited. Supported products for `depository` accounts are: Auth (`checking` and `savings` types only), Balance, Transactions, Identity, Payment Initiation, and Assets. | 
**credit** | **String** | A credit card type account. Supported products for `credit` accounts are: Balance, Transactions, Identity, and Liabilities. | 
**loan** | **String** | A loan type account. Supported products for `loan` accounts are: Balance, Liabilities, and Transactions. | 
**investment** | **String** | An investment account. Supported products for `investment` accounts are: Balance and Investments. In API versions 2018-05-22 and earlier, this type is called `brokerage`. | 
**other** | **String** | Other or unknown account type. Supported products for `other` accounts are: Balance, Transactions, Identity, and Assets. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


