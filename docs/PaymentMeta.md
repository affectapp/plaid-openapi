# PaymentMeta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reference_number** | Option<**String**> | The transaction reference number supplied by the financial institution. | 
**ppd_id** | Option<**String**> | The ACH PPD ID for the payer. | 
**payee** | Option<**String**> | For transfers, the party that is receiving the transaction. | 
**by_order_of** | Option<**String**> | The party initiating a wire transfer. Will be `null` if the transaction is not a wire transfer. | 
**payer** | Option<**String**> | For transfers, the party that is paying the transaction. | 
**payment_method** | Option<**String**> | The type of transfer, e.g. 'ACH' | 
**payment_processor** | Option<**String**> | The name of the payment processor | 
**reason** | Option<**String**> | The payer-supplied description of the transfer. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


