# Error

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error_type** | **String** | A broad categorization of the error. Safe for programmatic use. | 
**error_code** | **String** | The particular error code. Safe for programmatic use. | 
**error_message** | **String** | A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use. | 
**display_message** | Option<**String**> | A user-friendly representation of the error code. `null` if the error is not related to user action.  This may change over time and is not safe for programmatic use. | 
**request_id** | Option<**String**> | A unique ID identifying the request, to be used for troubleshooting purposes. This field will be omitted in errors provided by webhooks. | [optional]
**causes** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | In the Assets product, a request can pertain to more than one Item. If an error is returned for such a request, `causes` will return an array of errors containing a breakdown of these errors on the individual Item level, if any can be identified.  `causes` will only be provided for the `error_type` `ASSET_REPORT_ERROR`. `causes` will also not be populated inside an error nested within a `warning` object. | [optional]
**status** | Option<**f32**> | The HTTP status code associated with the error. This will only be returned in the response body when the error information is provided via a webhook. | [optional]
**documentation_url** | Option<**String**> | The URL of a Plaid documentation page with more information about the error | [optional]
**suggested_action** | Option<**String**> | Suggested steps for resolving the error | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


