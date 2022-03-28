# Enhancements

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**merchant_name** | Option<**String**> | The merchant name, as extracted by Plaid from the raw description. | [optional]
**check_number** | Option<**String**> | The check number of the transaction. This field is only populated for check transactions. | [optional]
**payment_channel** | [**crate::models::PaymentChannel**](PaymentChannel.md) |  | 
**category_id** | Option<**String**> | The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/#categoriesget). | 
**category** | **Vec<String>** | A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/#categoriesget). | 
**location** | [**crate::models::Location**](Location.md) |  | 
**personal_finance_category** | Option<[**crate::models::PersonalFinanceCategory**](PersonalFinanceCategory.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


