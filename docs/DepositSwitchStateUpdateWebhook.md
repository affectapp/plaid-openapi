# DepositSwitchStateUpdateWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_type** | Option<**String**> | `\"DEPOSIT_SWITCH\"` | [optional]
**webhook_code** | Option<**String**> | `\"SWITCH_STATE_UPDATE\"` | [optional]
**state** | Option<**String**> |  The state, or status, of the deposit switch.  `initialized`: The deposit switch has been initialized with the user entering the information required to submit the deposit switch request.  `processing`: The deposit switch request has been submitted and is being processed.  `completed`: The user's employer has fulfilled and completed the deposit switch request.  `error`: There was an error processing the deposit switch request.  For more information, see the [Deposit Switch API reference](/docs/deposit-switch/reference#deposit_switchget). | [optional]
**deposit_switch_id** | Option<**String**> | The ID of the deposit switch. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


