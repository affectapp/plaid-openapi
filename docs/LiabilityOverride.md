# LiabilityOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** | The type of the liability object, either `credit` or `student`. Mortgages are not currently supported in the custom Sandbox. | 
**purchase_apr** | **f64** | The purchase APR percentage value. For simplicity, this is the only interest rate used to calculate interest charges. Can only be set if `type` is `credit`. | 
**cash_apr** | **f64** | The cash APR percentage value. Can only be set if `type` is `credit`. | 
**balance_transfer_apr** | **f64** | The balance transfer APR percentage value. Can only be set if `type` is `credit`. Can only be set if `type` is `credit`. | 
**special_apr** | **f64** | The special APR percentage value. Can only be set if `type` is `credit`. | 
**last_payment_amount** | **f64** | Override the `last_payment_amount` field. Can only be set if `type` is `credit`. | 
**minimum_payment_amount** | **f64** | Override the `minimum_payment_amount` field. Can only be set if `type` is `credit` or `student`. | 
**is_overdue** | **bool** | Override the `is_overdue` field | 
**origination_date** | [**String**](string.md) | The date on which the loan was initially lent, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Can only be set if `type` is `student`. | 
**principal** | **f64** | The original loan principal. Can only be set if `type` is `student`. | 
**nominal_apr** | **f64** | The interest rate on the loan as a percentage. Can only be set if `type` is `student`. | 
**interest_capitalization_grace_period_months** | **f32** | If set, interest capitalization begins at the given number of months after loan origination. By default interest is never capitalized. Can only be set if `type` is `student`. | 
**repayment_model** | [**crate::models::StudentLoanRepaymentModel**](StudentLoanRepaymentModel.md) |  | 
**expected_payoff_date** | [**String**](string.md) | Override the `expected_payoff_date` field. Can only be set if `type` is `student`. | 
**guarantor** | **String** | Override the `guarantor` field. Can only be set if `type` is `student`. | 
**is_federal** | **bool** | Override the `is_federal` field. Can only be set if `type` is `student`. | 
**loan_name** | **String** | Override the `loan_name` field. Can only be set if `type` is `student`. | 
**loan_status** | [**crate::models::StudentLoanStatus**](StudentLoanStatus.md) |  | 
**payment_reference_number** | **String** | Override the `payment_reference_number` field. Can only be set if `type` is `student`. | 
**pslf_status** | [**crate::models::PslfStatus**](PSLFStatus.md) |  | 
**repayment_plan_description** | **String** | Override the `repayment_plan.description` field. Can only be set if `type` is `student`. | 
**repayment_plan_type** | **String** | Override the `repayment_plan.type` field. Can only be set if `type` is `student`. Possible values are: `\"extended graduated\"`, `\"extended standard\"`, `\"graduated\"`, `\"income-contingent repayment\"`, `\"income-based repayment\"`, `\"interest only\"`, `\"other\"`, `\"pay as you earn\"`, `\"revised pay as you earn\"`, or `\"standard\"`. | 
**sequence_number** | **String** | Override the `sequence_number` field. Can only be set if `type` is `student`. | 
**servicer_address** | [**crate::models::Address**](Address.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


