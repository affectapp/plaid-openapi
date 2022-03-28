# Security

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**security_id** | **String** | A unique, Plaid-specific identifier for the security, used to associate securities with holdings. Like all Plaid identifiers, the `security_id` is case sensitive. | 
**isin** | Option<**String**> | 12-character ISIN, a globally unique securities identifier. | 
**cusip** | Option<**String**> | 9-character CUSIP, an identifier assigned to North American securities. | 
**sedol** | Option<**String**> | 7-character SEDOL, an identifier assigned to securities in the UK. | 
**institution_security_id** | Option<**String**> | An identifier given to the security by the institution | 
**institution_id** | Option<**String**> | If `institution_security_id` is present, this field indicates the Plaid `institution_id` of the institution to whom the identifier belongs. | 
**proxy_security_id** | Option<**String**> | In certain cases, Plaid will provide the ID of another security whose performance resembles this security, typically when the original security has low volume, or when a private security can be modeled with a publicly traded security. | 
**name** | Option<**String**> | A descriptive name for the security, suitable for display. | 
**ticker_symbol** | Option<**String**> | The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available. | 
**is_cash_equivalent** | Option<**bool**> | Indicates that a security is a highly liquid asset and can be treated like cash. | 
**_type** | Option<**String**> | The security type of the holding. Valid security types are:  `cash`: Cash, currency, and money market funds  `derivative`: Options, warrants, and other derivative instruments  `equity`: Domestic and foreign equities  `etf`: Multi-asset exchange-traded investment funds  `fixed income`: Bonds and certificates of deposit (CDs)  `loan`: Loans and loan receivables.  `mutual fund`: Open- and closed-end vehicles pooling funds of multiple investors.  `other`: Unknown or other investment types | 
**close_price** | Option<**f64**> | Price of the security at the close of the previous trading session. `null` for non-public securities. If the security is a foreign currency or a cryptocurrency this field will be updated daily and will be priced in USD. | 
**close_price_as_of** | Option<[**String**](string.md)> | Date for which `close_price` is accurate. Always `null` if `close_price` is `null`. | 
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the price given. Always `null` if `unofficial_currency_code` is non-`null`. | 
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


