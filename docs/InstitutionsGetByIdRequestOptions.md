# InstitutionsGetByIdRequestOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**include_optional_metadata** | Option<**bool**> | When `true`, return an institution's logo, brand color, and URL. When available, the bank's logo is returned as a base64 encoded 152x152 PNG, the brand color is in hexadecimal format. The default value is `false`.  Note that Plaid does not own any of the logos shared by the API and that by accessing or using these logos, you agree that you are doing so at your own risk and will, if necessary, obtain all required permissions from the appropriate rights holders and adhere to any applicable usage guidelines. Plaid disclaims all express or implied warranties with respect to the logos. | [optional][default to false]
**include_status** | Option<**bool**> | If `true`, the response will include status information about the institution. Default value is `false`. | [optional][default to false]
**include_auth_metadata** | Option<**bool**> | When `true`, returns metadata related to the Auth product indicating which auth methods are supported. | [optional][default to false]
**include_payment_initiation_metadata** | Option<**bool**> | When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


