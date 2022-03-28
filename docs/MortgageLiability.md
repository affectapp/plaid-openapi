# MortgageLiability

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the account that this liability belongs to. | 
**account_number** | **String** | The account number of the loan. | 
**current_late_fee** | Option<**f64**> | The current outstanding amount charged for late payment. | 
**escrow_balance** | Option<**f64**> | Total amount held in escrow to pay taxes and insurance on behalf of the borrower. | 
**has_pmi** | Option<**bool**> | Indicates whether the borrower has private mortgage insurance in effect. | 
**has_prepayment_penalty** | Option<**bool**> | Indicates whether the borrower will pay a penalty for early payoff of mortgage. | 
**interest_rate** | [**crate::models::MortgageInterestRate**](MortgageInterestRate.md) |  | 
**last_payment_amount** | Option<**f64**> | The amount of the last payment. | 
**last_payment_date** | Option<[**String**](string.md)> | The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). | 
**loan_type_description** | Option<**String**> | Description of the type of loan, for example `conventional`, `fixed`, or `variable`. This field is provided directly from the loan servicer and does not have an enumerated set of possible values. | 
**loan_term** | Option<**String**> | Full duration of mortgage as at origination (e.g. `10 year`). | 
**maturity_date** | Option<[**String**](string.md)> | Original date on which mortgage is due in full. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). | 
**next_monthly_payment** | Option<**f64**> | The amount of the next payment. | 
**next_payment_due_date** | Option<[**String**](string.md)> | The due date for the next payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). | 
**origination_date** | Option<[**String**](string.md)> | The date on which the loan was initially lent. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). | 
**origination_principal_amount** | Option<**f64**> | The original principal balance of the mortgage. | 
**past_due_amount** | Option<**f64**> | Amount of loan (principal + interest) past due for payment. | 
**property_address** | [**crate::models::MortgagePropertyAddress**](MortgagePropertyAddress.md) |  | 
**ytd_interest_paid** | Option<**f64**> | The year to date (YTD) interest paid. | 
**ytd_principal_paid** | Option<**f64**> | The YTD principal paid. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


