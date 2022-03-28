# StudentLoan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The ID of the account that this liability belongs to. | 
**account_number** | Option<**String**> | The account number of the loan. For some institutions, this may be a masked version of the number (e.g., the last 4 digits instead of the entire number). | 
**disbursement_dates** | Option<[**Vec<String>**](string.md)> | The dates on which loaned funds were disbursed or will be disbursed. These are often in the past. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). | 
**expected_payoff_date** | Option<[**String**](string.md)> | The date when the student loan is expected to be paid off. Availability for this field is limited. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). | 
**guarantor** | Option<**String**> | The guarantor of the student loan. | 
**interest_rate_percentage** | **f64** | The interest rate on the loan as a percentage. | 
**is_overdue** | Option<**bool**> | `true` if a payment is currently overdue. Availability for this field is limited. | 
**last_payment_amount** | Option<**f64**> | The amount of the last payment. | 
**last_payment_date** | Option<[**String**](string.md)> | The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). | 
**last_statement_issue_date** | Option<[**String**](string.md)> | The date of the last statement. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). | 
**loan_name** | Option<**String**> | The type of loan, e.g., \"Consolidation Loans\". | 
**loan_status** | [**crate::models::StudentLoanStatus**](StudentLoanStatus.md) |  | 
**minimum_payment_amount** | Option<**f64**> | The minimum payment due for the next billing cycle. There are some exceptions: Some institutions require a minimum payment across all loans associated with an account number. Our API presents that same minimum payment amount on each loan. The institutions that do this are: Great Lakes ( `ins_116861`), Firstmark (`ins_116295`), Commonbond Firstmark Services (`ins_116950`), Nelnet (`ins_116528`), EdFinancial Services (`ins_116304`), Granite State (`ins_116308`), and Oklahoma Student Loan Authority (`ins_116945`). Firstmark (`ins_116295` ) and Navient (`ins_116248`) will display as $0 if there is an autopay program in effect. | 
**next_payment_due_date** | Option<[**String**](string.md)> | The due date for the next payment. The due date is `null` if a payment is not expected. A payment is not expected if `loan_status.type` is `deferment`, `in_school`, `consolidated`, `paid in full`, or `transferred`. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). | 
**origination_date** | Option<[**String**](string.md)> | The date on which the loan was initially lent. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).  | 
**origination_principal_amount** | Option<**f64**> | The original principal balance of the loan. | 
**outstanding_interest_amount** | Option<**f64**> | The total dollar amount of the accrued interest balance. For Sallie Mae ( `ins_116944`), this amount is included in the current balance of the loan, so this field will return as `null`. | 
**payment_reference_number** | Option<**String**> | The relevant account number that should be used to reference this loan for payments. In the majority of cases, `payment_reference_number` will match a`ccount_number,` but in some institutions, such as Great Lakes (`ins_116861`), it will be different. | 
**pslf_status** | [**crate::models::PslfStatus**](PSLFStatus.md) |  | 
**repayment_plan** | [**crate::models::StudentRepaymentPlan**](StudentRepaymentPlan.md) |  | 
**sequence_number** | Option<**String**> | The sequence number of the student loan. Heartland ECSI (`ins_116948`) does not make this field available. | 
**servicer_address** | [**crate::models::ServicerAddressData**](ServicerAddressData.md) |  | 
**ytd_interest_paid** | Option<**f64**> | The year to date (YTD) interest paid. Availability for this field is limited. | 
**ytd_principal_paid** | Option<**f64**> | The year to date (YTD) principal paid. Availability for this field is limited. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


