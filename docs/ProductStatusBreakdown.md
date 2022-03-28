# ProductStatusBreakdown

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | **f64** | The percentage of login attempts that are successful, expressed as a decimal. | 
**error_plaid** | **f64** | The percentage of logins that are failing due to an internal Plaid issue, expressed as a decimal.  | 
**error_institution** | **f64** | The percentage of logins that are failing due to an issue in the institution's system, expressed as a decimal. | 
**refresh_interval** | Option<**String**> | The `refresh_interval` may be `DELAYED` or `STOPPED` even when the success rate is high. This value is only returned for Transactions status breakdowns. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


