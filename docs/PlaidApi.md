# \PlaidApi

All URIs are relative to *https://production.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accounts_balance_get**](PlaidApi.md#accounts_balance_get) | **POST** /accounts/balance/get | Retrieve real-time balance data
[**accounts_get**](PlaidApi.md#accounts_get) | **POST** /accounts/get | Retrieve accounts
[**application_get**](PlaidApi.md#application_get) | **POST** /application/get | Retrieve information about a Plaid application
[**asset_report_audit_copy_create**](PlaidApi.md#asset_report_audit_copy_create) | **POST** /asset_report/audit_copy/create | Create Asset Report Audit Copy
[**asset_report_audit_copy_get**](PlaidApi.md#asset_report_audit_copy_get) | **POST** /asset_report/audit_copy/get | Retrieve an Asset Report Audit Copy
[**asset_report_audit_copy_remove**](PlaidApi.md#asset_report_audit_copy_remove) | **POST** /asset_report/audit_copy/remove | Remove Asset Report Audit Copy
[**asset_report_create**](PlaidApi.md#asset_report_create) | **POST** /asset_report/create | Create an Asset Report
[**asset_report_filter**](PlaidApi.md#asset_report_filter) | **POST** /asset_report/filter | Filter Asset Report
[**asset_report_get**](PlaidApi.md#asset_report_get) | **POST** /asset_report/get | Retrieve an Asset Report
[**asset_report_pdf_get**](PlaidApi.md#asset_report_pdf_get) | **POST** /asset_report/pdf/get | Retrieve a PDF Asset Report
[**asset_report_refresh**](PlaidApi.md#asset_report_refresh) | **POST** /asset_report/refresh | Refresh an Asset Report
[**asset_report_relay_create**](PlaidApi.md#asset_report_relay_create) | **POST** /asset_report/relay/create | Create an `asset_relay_token` to share an Asset Report with a partner client
[**asset_report_relay_get**](PlaidApi.md#asset_report_relay_get) | **POST** /asset_report/relay/get | Retrieve an Asset Report that was shared with you
[**asset_report_relay_refresh**](PlaidApi.md#asset_report_relay_refresh) | **POST** /asset_report/relay/refresh | Refresh a Relay Token's Asset Report
[**asset_report_relay_remove**](PlaidApi.md#asset_report_relay_remove) | **POST** /asset_report/relay/remove | Remove Asset Report Relay Token
[**asset_report_remove**](PlaidApi.md#asset_report_remove) | **POST** /asset_report/remove | Delete an Asset Report
[**auth_get**](PlaidApi.md#auth_get) | **POST** /auth/get | Retrieve auth data
[**bank_transfer_balance_get**](PlaidApi.md#bank_transfer_balance_get) | **POST** /bank_transfer/balance/get | Get balance of your Bank Transfer account
[**bank_transfer_cancel**](PlaidApi.md#bank_transfer_cancel) | **POST** /bank_transfer/cancel | Cancel a bank transfer
[**bank_transfer_create**](PlaidApi.md#bank_transfer_create) | **POST** /bank_transfer/create | Create a bank transfer
[**bank_transfer_event_list**](PlaidApi.md#bank_transfer_event_list) | **POST** /bank_transfer/event/list | List bank transfer events
[**bank_transfer_event_sync**](PlaidApi.md#bank_transfer_event_sync) | **POST** /bank_transfer/event/sync | Sync bank transfer events
[**bank_transfer_get**](PlaidApi.md#bank_transfer_get) | **POST** /bank_transfer/get | Retrieve a bank transfer
[**bank_transfer_list**](PlaidApi.md#bank_transfer_list) | **POST** /bank_transfer/list | List bank transfers
[**bank_transfer_migrate_account**](PlaidApi.md#bank_transfer_migrate_account) | **POST** /bank_transfer/migrate_account | Migrate account into Bank Transfers
[**bank_transfer_sweep_get**](PlaidApi.md#bank_transfer_sweep_get) | **POST** /bank_transfer/sweep/get | Retrieve a sweep
[**bank_transfer_sweep_list**](PlaidApi.md#bank_transfer_sweep_list) | **POST** /bank_transfer/sweep/list | List sweeps
[**categories_get**](PlaidApi.md#categories_get) | **POST** /categories/get | Get Categories
[**create_payment_token**](PlaidApi.md#create_payment_token) | **POST** /payment_initiation/payment/token/create | Create payment token
[**credit_bank_income_get**](PlaidApi.md#credit_bank_income_get) | **POST** /credit/bank_income/get | Retrieve information from the bank accounts used for income verification
[**deposit_switch_alt_create**](PlaidApi.md#deposit_switch_alt_create) | **POST** /deposit_switch/alt/create | Create a deposit switch without using Plaid Exchange
[**deposit_switch_create**](PlaidApi.md#deposit_switch_create) | **POST** /deposit_switch/create | Create a deposit switch
[**deposit_switch_get**](PlaidApi.md#deposit_switch_get) | **POST** /deposit_switch/get | Retrieve a deposit switch
[**deposit_switch_token_create**](PlaidApi.md#deposit_switch_token_create) | **POST** /deposit_switch/token/create | Create a deposit switch token
[**employers_search**](PlaidApi.md#employers_search) | **POST** /employers/search | Search employer database
[**employment_verification_get**](PlaidApi.md#employment_verification_get) | **POST** /employment/verification/get | Retrieve a summary of an individual's employment information
[**identity_get**](PlaidApi.md#identity_get) | **POST** /identity/get | Retrieve identity data
[**income_verification_create**](PlaidApi.md#income_verification_create) | **POST** /income/verification/create | (Deprecated) Create an income verification instance
[**income_verification_documents_download**](PlaidApi.md#income_verification_documents_download) | **POST** /income/verification/documents/download | Download the original documents used for income verification
[**income_verification_paystubs_get**](PlaidApi.md#income_verification_paystubs_get) | **POST** /income/verification/paystubs/get | Retrieve information from the paystubs used for income verification
[**income_verification_precheck**](PlaidApi.md#income_verification_precheck) | **POST** /income/verification/precheck | Check digital income verification eligibility and optimize conversion
[**income_verification_refresh**](PlaidApi.md#income_verification_refresh) | **POST** /income/verification/refresh | Refresh an income verification
[**income_verification_taxforms_get**](PlaidApi.md#income_verification_taxforms_get) | **POST** /income/verification/taxforms/get | Retrieve information from the tax documents used for income verification
[**institutions_get**](PlaidApi.md#institutions_get) | **POST** /institutions/get | Get details of all supported institutions
[**institutions_get_by_id**](PlaidApi.md#institutions_get_by_id) | **POST** /institutions/get_by_id | Get details of an institution
[**institutions_search**](PlaidApi.md#institutions_search) | **POST** /institutions/search | Search institutions
[**investments_holdings_get**](PlaidApi.md#investments_holdings_get) | **POST** /investments/holdings/get | Get Investment holdings
[**investments_transactions_get**](PlaidApi.md#investments_transactions_get) | **POST** /investments/transactions/get | Get investment transactions
[**item_access_token_invalidate**](PlaidApi.md#item_access_token_invalidate) | **POST** /item/access_token/invalidate | Invalidate access_token
[**item_application_list**](PlaidApi.md#item_application_list) | **POST** /item/application/list | List a user’s connected applications
[**item_application_scopes_update**](PlaidApi.md#item_application_scopes_update) | **POST** /item/application/scopes/update | Update the scopes of access for a particular application
[**item_create_public_token**](PlaidApi.md#item_create_public_token) | **POST** /item/public_token/create | Create public token
[**item_get**](PlaidApi.md#item_get) | **POST** /item/get | Retrieve an Item
[**item_import**](PlaidApi.md#item_import) | **POST** /item/import | Import Item
[**item_public_token_exchange**](PlaidApi.md#item_public_token_exchange) | **POST** /item/public_token/exchange | Exchange public token for an access token
[**item_remove**](PlaidApi.md#item_remove) | **POST** /item/remove | Remove an Item
[**item_webhook_update**](PlaidApi.md#item_webhook_update) | **POST** /item/webhook/update | Update Webhook URL
[**liabilities_get**](PlaidApi.md#liabilities_get) | **POST** /liabilities/get | Retrieve Liabilities data
[**link_token_create**](PlaidApi.md#link_token_create) | **POST** /link/token/create | Create Link Token
[**link_token_get**](PlaidApi.md#link_token_get) | **POST** /link/token/get | Get Link Token
[**payment_initiation_consent_create**](PlaidApi.md#payment_initiation_consent_create) | **POST** /payment_initiation/consent/create | Create payment consent
[**payment_initiation_consent_get**](PlaidApi.md#payment_initiation_consent_get) | **POST** /payment_initiation/consent/get | Get payment consent
[**payment_initiation_consent_payment_execute**](PlaidApi.md#payment_initiation_consent_payment_execute) | **POST** /payment_initiation/consent/payment/execute | Execute a single payment using consent
[**payment_initiation_consent_revoke**](PlaidApi.md#payment_initiation_consent_revoke) | **POST** /payment_initiation/consent/revoke | Revoke payment consent
[**payment_initiation_payment_create**](PlaidApi.md#payment_initiation_payment_create) | **POST** /payment_initiation/payment/create | Create a payment
[**payment_initiation_payment_get**](PlaidApi.md#payment_initiation_payment_get) | **POST** /payment_initiation/payment/get | Get payment details
[**payment_initiation_payment_list**](PlaidApi.md#payment_initiation_payment_list) | **POST** /payment_initiation/payment/list | List payments
[**payment_initiation_payment_reverse**](PlaidApi.md#payment_initiation_payment_reverse) | **POST** /payment_initiation/payment/reverse | Reverse an existing payment
[**payment_initiation_recipient_create**](PlaidApi.md#payment_initiation_recipient_create) | **POST** /payment_initiation/recipient/create | Create payment recipient
[**payment_initiation_recipient_get**](PlaidApi.md#payment_initiation_recipient_get) | **POST** /payment_initiation/recipient/get | Get payment recipient
[**payment_initiation_recipient_list**](PlaidApi.md#payment_initiation_recipient_list) | **POST** /payment_initiation/recipient/list | List payment recipients
[**processor_apex_processor_token_create**](PlaidApi.md#processor_apex_processor_token_create) | **POST** /processor/apex/processor_token/create | Create Apex bank account token
[**processor_auth_get**](PlaidApi.md#processor_auth_get) | **POST** /processor/auth/get | Retrieve Auth data
[**processor_balance_get**](PlaidApi.md#processor_balance_get) | **POST** /processor/balance/get | Retrieve Balance data
[**processor_bank_transfer_create**](PlaidApi.md#processor_bank_transfer_create) | **POST** /processor/bank_transfer/create | Create a bank transfer as a processor
[**processor_identity_get**](PlaidApi.md#processor_identity_get) | **POST** /processor/identity/get | Retrieve Identity data
[**processor_stripe_bank_account_token_create**](PlaidApi.md#processor_stripe_bank_account_token_create) | **POST** /processor/stripe/bank_account_token/create | Create Stripe bank account token
[**processor_token_create**](PlaidApi.md#processor_token_create) | **POST** /processor/token/create | Create processor token
[**sandbox_bank_transfer_fire_webhook**](PlaidApi.md#sandbox_bank_transfer_fire_webhook) | **POST** /sandbox/bank_transfer/fire_webhook | Manually fire a Bank Transfer webhook
[**sandbox_bank_transfer_simulate**](PlaidApi.md#sandbox_bank_transfer_simulate) | **POST** /sandbox/bank_transfer/simulate | Simulate a bank transfer event in Sandbox
[**sandbox_income_fire_webhook**](PlaidApi.md#sandbox_income_fire_webhook) | **POST** /sandbox/income/fire_webhook | Manually fire an Income webhook
[**sandbox_item_fire_webhook**](PlaidApi.md#sandbox_item_fire_webhook) | **POST** /sandbox/item/fire_webhook | Fire a test webhook
[**sandbox_item_reset_login**](PlaidApi.md#sandbox_item_reset_login) | **POST** /sandbox/item/reset_login | Force a Sandbox Item into an error state
[**sandbox_item_set_verification_status**](PlaidApi.md#sandbox_item_set_verification_status) | **POST** /sandbox/item/set_verification_status | Set verification status for Sandbox account
[**sandbox_oauth_select_accounts**](PlaidApi.md#sandbox_oauth_select_accounts) | **POST** /sandbox/oauth/select_accounts | Save the selected accounts when connecting to the Platypus Oauth institution
[**sandbox_processor_token_create**](PlaidApi.md#sandbox_processor_token_create) | **POST** /sandbox/processor_token/create | Create a test Item and processor token
[**sandbox_public_token_create**](PlaidApi.md#sandbox_public_token_create) | **POST** /sandbox/public_token/create | Create a test Item
[**sandbox_transfer_repayment_simulate**](PlaidApi.md#sandbox_transfer_repayment_simulate) | **POST** /sandbox/transfer/repayment/simulate | Trigger the creation of a repayment
[**sandbox_transfer_simulate**](PlaidApi.md#sandbox_transfer_simulate) | **POST** /sandbox/transfer/simulate | Simulate a transfer event in Sandbox
[**sandbox_transfer_sweep_simulate**](PlaidApi.md#sandbox_transfer_sweep_simulate) | **POST** /sandbox/transfer/sweep/simulate | Simulate creating a sweep
[**signal_decision_report**](PlaidApi.md#signal_decision_report) | **POST** /signal/decision/report | Report whether you initiated an ACH transaction
[**signal_evaluate**](PlaidApi.md#signal_evaluate) | **POST** /signal/evaluate | Evaluate a planned ACH transaction
[**signal_return_report**](PlaidApi.md#signal_return_report) | **POST** /signal/return/report | Report a return for an ACH transaction
[**transactions_enhance**](PlaidApi.md#transactions_enhance) | **POST** /beta/transactions/v1/enhance | enhance locally-held transaction data
[**transactions_get**](PlaidApi.md#transactions_get) | **POST** /transactions/get | Get transaction data
[**transactions_recurring_get**](PlaidApi.md#transactions_recurring_get) | **POST** /transactions/recurring/get | Get streams of recurring transactions
[**transactions_refresh**](PlaidApi.md#transactions_refresh) | **POST** /transactions/refresh | Refresh transaction data
[**transactions_rules_create**](PlaidApi.md#transactions_rules_create) | **POST** /beta/transactions/rules/create | Create transaction category rule
[**transactions_rules_list**](PlaidApi.md#transactions_rules_list) | **POST** /beta/transactions/rules/list | Return a list of rules created for the Item associated with the access token.
[**transactions_rules_remove**](PlaidApi.md#transactions_rules_remove) | **POST** /beta/transactions/rules/remove | Remove transaction rule
[**transactions_sync**](PlaidApi.md#transactions_sync) | **POST** /transactions/sync | Get incremental transaction updates on an Item
[**transfer_authorization_create**](PlaidApi.md#transfer_authorization_create) | **POST** /transfer/authorization/create | Create a transfer authorization
[**transfer_cancel**](PlaidApi.md#transfer_cancel) | **POST** /transfer/cancel | Cancel a transfer
[**transfer_create**](PlaidApi.md#transfer_create) | **POST** /transfer/create | Create a transfer
[**transfer_event_list**](PlaidApi.md#transfer_event_list) | **POST** /transfer/event/list | List transfer events
[**transfer_event_sync**](PlaidApi.md#transfer_event_sync) | **POST** /transfer/event/sync | Sync transfer events
[**transfer_get**](PlaidApi.md#transfer_get) | **POST** /transfer/get | Retrieve a transfer
[**transfer_intent_create**](PlaidApi.md#transfer_intent_create) | **POST** /transfer/intent/create | Create a transfer intent object to invoke the Transfer UI
[**transfer_intent_get**](PlaidApi.md#transfer_intent_get) | **POST** /transfer/intent/get | Retrieve more information about a transfer intent
[**transfer_list**](PlaidApi.md#transfer_list) | **POST** /transfer/list | List transfers
[**transfer_repayment_list**](PlaidApi.md#transfer_repayment_list) | **POST** /transfer/repayment/list | Lists historical repayments
[**transfer_repayment_return_list**](PlaidApi.md#transfer_repayment_return_list) | **POST** /transfer/repayment/return/list | List the returns included in a repayment
[**transfer_sweep_get**](PlaidApi.md#transfer_sweep_get) | **POST** /transfer/sweep/get | Retrieve a sweep
[**transfer_sweep_list**](PlaidApi.md#transfer_sweep_list) | **POST** /transfer/sweep/list | List sweeps
[**user_create**](PlaidApi.md#user_create) | **POST** /user/create | Create user
[**wallet_get**](PlaidApi.md#wallet_get) | **POST** /wallet/get | Fetch an e-wallet
[**wallet_transaction_execute**](PlaidApi.md#wallet_transaction_execute) | **POST** /wallet/transaction/execute | Execute a transaction using an e-wallet
[**wallet_transactions_list**](PlaidApi.md#wallet_transactions_list) | **POST** /wallet/transactions/list | List e-wallet transactions
[**webhook_verification_key_get**](PlaidApi.md#webhook_verification_key_get) | **POST** /webhook_verification_key/get | Get webhook verification key



## accounts_balance_get

> crate::models::AccountsGetResponse accounts_balance_get(accounts_balance_get_request)
Retrieve real-time balance data

The `/accounts/balance/get` endpoint returns the real-time balance for each of an Item's accounts. While other endpoints may return a balance object, only `/accounts/balance/get` forces the available and current balance fields to be refreshed rather than cached. This endpoint can be used for existing Items that were added via any of Plaid’s other products. This endpoint can be used as long as Link has been initialized with any other product, `balance` itself is not a product that can be used to initialize Link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accounts_balance_get_request** | [**AccountsBalanceGetRequest**](AccountsBalanceGetRequest.md) |  | [required] |

### Return type

[**crate::models::AccountsGetResponse**](AccountsGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_get

> crate::models::AccountsGetResponse accounts_get(accounts_get_request)
Retrieve accounts

The `/accounts/get` endpoint can be used to retrieve a list of accounts associated with any linked Item. Plaid will only return active bank accounts — that is, accounts that are not closed and are capable of carrying a balance. This endpoint only returns accounts that were permissioned by the user when they initially created the Item. If a user creates a new account after the initial link, you can capture this event through the [`NEW_ACCOUNTS_AVAILABLE`](https://plaid.com/docs/api/items/#new_accounts_available) webhook and then use Link's [update mode](https://plaid.com/docs/link/update-mode/) to request that the user share this new account with you.  This endpoint retrieves cached information, rather than extracting fresh information from the institution. As a result, balances returned may not be up-to-date; for realtime balance information, use `/accounts/balance/get` instead. Note that some information is nullable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accounts_get_request** | [**AccountsGetRequest**](AccountsGetRequest.md) |  | [required] |

### Return type

[**crate::models::AccountsGetResponse**](AccountsGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## application_get

> crate::models::ApplicationGetResponse application_get(application_get_request)
Retrieve information about a Plaid application

Allows financial institutions to retrieve information about Plaid clients for the purpose of building control-tower experiences

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_get_request** | [**ApplicationGetRequest**](ApplicationGetRequest.md) |  | [required] |

### Return type

[**crate::models::ApplicationGetResponse**](ApplicationGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_audit_copy_create

> crate::models::AssetReportAuditCopyCreateResponse asset_report_audit_copy_create(asset_report_audit_copy_create_request)
Create Asset Report Audit Copy

Plaid can provide an Audit Copy of any Asset Report directly to a participating third party on your behalf. For example, Plaid can supply an Audit Copy directly to Fannie Mae on your behalf if you participate in the Day 1 Certainty™ program. An Audit Copy contains the same underlying data as the Asset Report.  To grant access to an Audit Copy, use the `/asset_report/audit_copy/create` endpoint to create an `audit_copy_token` and then pass that token to the third party who needs access. Each third party has its own `auditor_id`, for example `fannie_mae`. You’ll need to create a separate Audit Copy for each third party to whom you want to grant access to the Report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_audit_copy_create_request** | [**AssetReportAuditCopyCreateRequest**](AssetReportAuditCopyCreateRequest.md) |  | [required] |

### Return type

[**crate::models::AssetReportAuditCopyCreateResponse**](AssetReportAuditCopyCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_audit_copy_get

> crate::models::AssetReportGetResponse asset_report_audit_copy_get(asset_report_audit_copy_get_request)
Retrieve an Asset Report Audit Copy

`/asset_report/audit_copy/get` allows auditors to get a copy of an Asset Report that was previously shared via the `/asset_report/audit_copy/create` endpoint.  The caller of `/asset_report/audit_copy/create` must provide the `audit_copy_token` to the auditor.  This token can then be used to call `/asset_report/audit_copy/create`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_audit_copy_get_request** | [**AssetReportAuditCopyGetRequest**](AssetReportAuditCopyGetRequest.md) |  | [required] |

### Return type

[**crate::models::AssetReportGetResponse**](AssetReportGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_audit_copy_remove

> crate::models::AssetReportAuditCopyRemoveResponse asset_report_audit_copy_remove(asset_report_audit_copy_remove_request)
Remove Asset Report Audit Copy

The `/asset_report/audit_copy/remove` endpoint allows you to remove an Audit Copy. Removing an Audit Copy invalidates the `audit_copy_token` associated with it, meaning both you and any third parties holding the token will no longer be able to use it to access Report data. Items associated with the Asset Report, the Asset Report itself and other Audit Copies of it are not affected and will remain accessible after removing the given Audit Copy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_audit_copy_remove_request** | [**AssetReportAuditCopyRemoveRequest**](AssetReportAuditCopyRemoveRequest.md) |  | [required] |

### Return type

[**crate::models::AssetReportAuditCopyRemoveResponse**](AssetReportAuditCopyRemoveResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_create

> crate::models::AssetReportCreateResponse asset_report_create(asset_report_create_request)
Create an Asset Report

The `/asset_report/create` endpoint initiates the process of creating an Asset Report, which can then be retrieved by passing the `asset_report_token` return value to the `/asset_report/get` or `/asset_report/pdf/get` endpoints.  The Asset Report takes some time to be created and is not available immediately after calling `/asset_report/create`. When the Asset Report is ready to be retrieved using `/asset_report/get` or `/asset_report/pdf/get`, Plaid will fire a `PRODUCT_READY` webhook. For full details of the webhook schema, see [Asset Report webhooks](https://plaid.com/docs/api/products/assets/#webhooks).  The `/asset_report/create` endpoint creates an Asset Report at a moment in time. Asset Reports are immutable. To get an updated Asset Report, use the `/asset_report/refresh` endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_create_request** | [**AssetReportCreateRequest**](AssetReportCreateRequest.md) |  | [required] |

### Return type

[**crate::models::AssetReportCreateResponse**](AssetReportCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_filter

> crate::models::AssetReportFilterResponse asset_report_filter(asset_report_filter_request)
Filter Asset Report

By default, an Asset Report will contain all of the accounts on a given Item. In some cases, you may not want the Asset Report to contain all accounts. For example, you might have the end user choose which accounts are relevant in Link using the Account Select view, which you can enable in the dashboard. Or, you might always exclude certain account types or subtypes, which you can identify by using the `/accounts/get` endpoint. To narrow an Asset Report to only a subset of accounts, use the `/asset_report/filter` endpoint.  To exclude certain Accounts from an Asset Report, first use the `/asset_report/create` endpoint to create the report, then send the `asset_report_token` along with a list of `account_ids` to exclude to the `/asset_report/filter` endpoint, to create a new Asset Report which contains only a subset of the original Asset Report's data.  Because Asset Reports are immutable, calling `/asset_report/filter` does not alter the original Asset Report in any way; rather, `/asset_report/filter` creates a new Asset Report with a new token and id. Asset Reports created via `/asset_report/filter` do not contain new Asset data, and are not billed.  Plaid will fire a [`PRODUCT_READY`](https://plaid.com/docs/api/products/assets/#product_ready) webhook once generation of the filtered Asset Report has completed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_filter_request** | [**AssetReportFilterRequest**](AssetReportFilterRequest.md) |  | [required] |

### Return type

[**crate::models::AssetReportFilterResponse**](AssetReportFilterResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_get

> crate::models::AssetReportGetResponse asset_report_get(asset_report_get_request)
Retrieve an Asset Report

The `/asset_report/get` endpoint retrieves the Asset Report in JSON format. Before calling `/asset_report/get`, you must first create the Asset Report using `/asset_report/create` (or filter an Asset Report using `/asset_report/filter`) and then wait for the [`PRODUCT_READY`](https://plaid.com/docs/api/products/assets/#product_ready) webhook to fire, indicating that the Report is ready to be retrieved.  By default, an Asset Report includes transaction descriptions as returned by the bank, as opposed to parsed and categorized by Plaid. You can also receive cleaned and categorized transactions, as well as additional insights like merchant name or location information. We call this an Asset Report with Insights. An Asset Report with Insights provides transaction category, location, and merchant information in addition to the transaction strings provided in a standard Asset Report.  To retrieve an Asset Report with Insights, call the `/asset_report/get` endpoint with `include_insights` set to `true`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_get_request** | [**AssetReportGetRequest**](AssetReportGetRequest.md) |  | [required] |

### Return type

[**crate::models::AssetReportGetResponse**](AssetReportGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_pdf_get

> std::path::PathBuf asset_report_pdf_get(asset_report_pdf_get_request)
Retrieve a PDF Asset Report

The `/asset_report/pdf/get` endpoint retrieves the Asset Report in PDF format. Before calling `/asset_report/pdf/get`, you must first create the Asset Report using `/asset_report/create` (or filter an Asset Report using `/asset_report/filter`) and then wait for the [`PRODUCT_READY`](https://plaid.com/docs/api/products/assets/#product_ready) webhook to fire, indicating that the Report is ready to be retrieved.  The response to `/asset_report/pdf/get` is the PDF binary data. The `request_id`  is returned in the `Plaid-Request-ID` header.  [View a sample PDF Asset Report](https://plaid.com/documents/sample-asset-report.pdf).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_pdf_get_request** | [**AssetReportPdfGetRequest**](AssetReportPdfGetRequest.md) |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/pdf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_refresh

> crate::models::AssetReportRefreshResponse asset_report_refresh(asset_report_refresh_request)
Refresh an Asset Report

An Asset Report is an immutable snapshot of a user's assets. In order to \"refresh\" an Asset Report you created previously, you can use the `/asset_report/refresh` endpoint to create a new Asset Report based on the old one, but with the most recent data available.  The new Asset Report will contain the same Items as the original Report, as well as the same filters applied by any call to `/asset_report/filter`. By default, the new Asset Report will also use the same parameters you submitted with your original `/asset_report/create` request, but the original `days_requested` value and the values of any parameters in the `options` object can be overridden with new values. To change these arguments, simply supply new values for them in your request to `/asset_report/refresh`. Submit an empty string (\"\") for any previously-populated fields you would like set as empty.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_refresh_request** | [**AssetReportRefreshRequest**](AssetReportRefreshRequest.md) |  | [required] |

### Return type

[**crate::models::AssetReportRefreshResponse**](AssetReportRefreshResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_relay_create

> crate::models::AssetReportRelayCreateResponse asset_report_relay_create(asset_report_relay_create_request)
Create an `asset_relay_token` to share an Asset Report with a partner client

Plaid can share an Asset Report directly with a participating third party on your behalf. The shared Asset Report is the exact same Asset Report originally created in `/asset_report/create`.  To grant access to an Asset Report to a third party, use the `/asset_report/relay/create` endpoint to create an `asset_relay_token` and then pass that token to the third party who needs access. Each third party has its own `secondary_client_id`, for example `ce5bd328dcd34123456`. You'll need to create a separate `asset_relay_token` for each third party to whom you want to grant access to the Report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_relay_create_request** | [**AssetReportRelayCreateRequest**](AssetReportRelayCreateRequest.md) |  | [required] |

### Return type

[**crate::models::AssetReportRelayCreateResponse**](AssetReportRelayCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_relay_get

> crate::models::AssetReportGetResponse asset_report_relay_get(asset_report_relay_get_request)
Retrieve an Asset Report that was shared with you

`/asset_report/relay/get` allows third parties to get an Asset Report that was shared with them, using an `asset_relay_token` that was created by the report owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_relay_get_request** | [**AssetReportRelayGetRequest**](AssetReportRelayGetRequest.md) |  | [required] |

### Return type

[**crate::models::AssetReportGetResponse**](AssetReportGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_relay_refresh

> crate::models::AssetReportRelayRefreshResponse asset_report_relay_refresh(asset_report_relay_refresh_request)
Refresh a Relay Token's Asset Report

The `/asset_report/relay/refresh` endpoint allows third parties to refresh an Asset Report that was relayed to them, using an `asset_relay_token` that was created by the report owner. A new Asset Report will be created based on the old one, but with the most recent data available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_relay_refresh_request** | [**AssetReportRelayRefreshRequest**](AssetReportRelayRefreshRequest.md) |  | [required] |

### Return type

[**crate::models::AssetReportRelayRefreshResponse**](AssetReportRelayRefreshResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_relay_remove

> crate::models::AssetReportRelayRemoveResponse asset_report_relay_remove(asset_report_relay_remove_request)
Remove Asset Report Relay Token

The `/asset_report/relay/remove` endpoint allows you to invalidate an `asset_relay_token`, meaning the third party holding the token will no longer be able to use it to access the Asset Report to which the `asset_relay_token` gives access to. The Asset Report, Items associated with it, and other Asset Relay Tokens that provide access to the same Asset Report are not affected and will remain accessible after removing the given `asset_relay_token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_relay_remove_request** | [**AssetReportRelayRemoveRequest**](AssetReportRelayRemoveRequest.md) |  | [required] |

### Return type

[**crate::models::AssetReportRelayRemoveResponse**](AssetReportRelayRemoveResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## asset_report_remove

> crate::models::AssetReportRemoveResponse asset_report_remove(asset_report_remove_request)
Delete an Asset Report

The `/item/remove` endpoint allows you to invalidate an `access_token`, meaning you will not be able to create new Asset Reports with it. Removing an Item does not affect any Asset Reports or Audit Copies you have already created, which will remain accessible until you remove them specifically.  The `/asset_report/remove` endpoint allows you to remove an Asset Report. Removing an Asset Report invalidates its `asset_report_token`, meaning you will no longer be able to use it to access Report data or create new Audit Copies. Removing an Asset Report does not affect the underlying Items, but does invalidate any `audit_copy_tokens` associated with the Asset Report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_report_remove_request** | [**AssetReportRemoveRequest**](AssetReportRemoveRequest.md) |  | [required] |

### Return type

[**crate::models::AssetReportRemoveResponse**](AssetReportRemoveResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_get

> crate::models::AuthGetResponse auth_get(auth_get_request)
Retrieve auth data

The `/auth/get` endpoint returns the bank account and bank identification numbers (such as routing numbers, for US accounts) associated with an Item's checking and savings accounts, along with high-level account data and balances when available.  Note: This request may take some time to complete if `auth` was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the data.  Also note that `/auth/get` will not return data for any new accounts opened after the Item was created. To obtain data for new accounts, create a new Item.  Versioning note: In API version 2017-03-08, the schema of the `numbers` object returned by this endpoint is substantially different. For details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2018-05-22).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_get_request** | [**AuthGetRequest**](AuthGetRequest.md) |  | [required] |

### Return type

[**crate::models::AuthGetResponse**](AuthGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_transfer_balance_get

> crate::models::BankTransferBalanceGetResponse bank_transfer_balance_get(bank_transfer_balance_get_request)
Get balance of your Bank Transfer account

Use the `/bank_transfer/balance/get` endpoint to see the available balance in your bank transfer account. Debit transfers increase this balance once their status is posted. Credit transfers decrease this balance when they are created.  The transactable balance shows the amount in your account that you are able to use for transfers, and is essentially your available balance minus your minimum balance.  Note that this endpoint can only be used with FBO accounts, when using Bank Transfers in the Full Service configuration. It cannot be used on your own account when using Bank Transfers in the BTS Platform configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bank_transfer_balance_get_request** | [**BankTransferBalanceGetRequest**](BankTransferBalanceGetRequest.md) |  | [required] |

### Return type

[**crate::models::BankTransferBalanceGetResponse**](BankTransferBalanceGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_transfer_cancel

> crate::models::BankTransferCancelResponse bank_transfer_cancel(bank_transfer_cancel_request)
Cancel a bank transfer

Use the `/bank_transfer/cancel` endpoint to cancel a bank transfer.  A transfer is eligible for cancelation if the `cancellable` property returned by `/bank_transfer/get` is `true`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bank_transfer_cancel_request** | [**BankTransferCancelRequest**](BankTransferCancelRequest.md) |  | [required] |

### Return type

[**crate::models::BankTransferCancelResponse**](BankTransferCancelResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_transfer_create

> crate::models::BankTransferCreateResponse bank_transfer_create(bank_transfer_create_request)
Create a bank transfer

Use the `/bank_transfer/create` endpoint to initiate a new bank transfer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bank_transfer_create_request** | [**BankTransferCreateRequest**](BankTransferCreateRequest.md) |  | [required] |

### Return type

[**crate::models::BankTransferCreateResponse**](BankTransferCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_transfer_event_list

> crate::models::BankTransferEventListResponse bank_transfer_event_list(bank_transfer_event_list_request)
List bank transfer events

Use the `/bank_transfer/event/list` endpoint to get a list of bank transfer events based on specified filter criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bank_transfer_event_list_request** | [**BankTransferEventListRequest**](BankTransferEventListRequest.md) |  | [required] |

### Return type

[**crate::models::BankTransferEventListResponse**](BankTransferEventListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_transfer_event_sync

> crate::models::BankTransferEventSyncResponse bank_transfer_event_sync(bank_transfer_event_sync_request)
Sync bank transfer events

`/bank_transfer/event/sync` allows you to request up to the next 25 bank transfer events that happened after a specific `event_id`. Use the `/bank_transfer/event/sync` endpoint to guarantee you have seen all bank transfer events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bank_transfer_event_sync_request** | [**BankTransferEventSyncRequest**](BankTransferEventSyncRequest.md) |  | [required] |

### Return type

[**crate::models::BankTransferEventSyncResponse**](BankTransferEventSyncResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_transfer_get

> crate::models::BankTransferGetResponse bank_transfer_get(bank_transfer_get_request)
Retrieve a bank transfer

The `/bank_transfer/get` fetches information about the bank transfer corresponding to the given `bank_transfer_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bank_transfer_get_request** | [**BankTransferGetRequest**](BankTransferGetRequest.md) |  | [required] |

### Return type

[**crate::models::BankTransferGetResponse**](BankTransferGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_transfer_list

> crate::models::BankTransferListResponse bank_transfer_list(bank_transfer_list_request)
List bank transfers

Use the `/bank_transfer/list` endpoint to see a list of all your bank transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired bank transfers. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bank_transfer_list_request** | [**BankTransferListRequest**](BankTransferListRequest.md) |  | [required] |

### Return type

[**crate::models::BankTransferListResponse**](BankTransferListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_transfer_migrate_account

> crate::models::BankTransferMigrateAccountResponse bank_transfer_migrate_account(bank_transfer_migrate_account_request)
Migrate account into Bank Transfers

As an alternative to adding Items via Link, you can also use the `/bank_transfer/migrate_account` endpoint to migrate known account and routing numbers to Plaid Items.  Note that Items created in this way are not compatible with endpoints for other products, such as `/accounts/balance/get`, and can only be used with Bank Transfer endpoints.  If you require access to other endpoints, create the Item through Link instead.  Access to `/bank_transfer/migrate_account` is not enabled by default; to obtain access, contact your Plaid Account Manager.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bank_transfer_migrate_account_request** | [**BankTransferMigrateAccountRequest**](BankTransferMigrateAccountRequest.md) |  | [required] |

### Return type

[**crate::models::BankTransferMigrateAccountResponse**](BankTransferMigrateAccountResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_transfer_sweep_get

> crate::models::BankTransferSweepGetResponse bank_transfer_sweep_get(bank_transfer_sweep_get_request)
Retrieve a sweep

The `/bank_transfer/sweep/get` endpoint fetches information about the sweep corresponding to the given `sweep_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bank_transfer_sweep_get_request** | [**BankTransferSweepGetRequest**](BankTransferSweepGetRequest.md) |  | [required] |

### Return type

[**crate::models::BankTransferSweepGetResponse**](BankTransferSweepGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_transfer_sweep_list

> crate::models::BankTransferSweepListResponse bank_transfer_sweep_list(bank_transfer_sweep_list_request)
List sweeps

The `/bank_transfer/sweep/list` endpoint fetches information about the sweeps matching the given filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bank_transfer_sweep_list_request** | [**BankTransferSweepListRequest**](BankTransferSweepListRequest.md) |  | [required] |

### Return type

[**crate::models::BankTransferSweepListResponse**](BankTransferSweepListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## categories_get

> crate::models::CategoriesGetResponse categories_get(body)
Get Categories

Send a request to the `/categories/get` endpoint to get detailed information on categories returned by Plaid. This endpoint does not require authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::CategoriesGetResponse**](CategoriesGetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_payment_token

> crate::models::PaymentInitiationPaymentTokenCreateResponse create_payment_token(payment_initiation_payment_token_create_request)
Create payment token

The `/payment_initiation/payment/token/create` endpoint has been deprecated. New Plaid customers will be unable to use this endpoint, and existing customers are encouraged to migrate to the newer, `link_token`-based flow. The recommended flow is to provide the `payment_id` to `/link/token/create`, which returns a `link_token` used to initialize Link.  The `/payment_initiation/payment/token/create` is used to create a `payment_token`, which can then be used in Link initialization to enter a payment initiation flow. You can only use a `payment_token` once. If this attempt fails, the end user aborts the flow, or the token expires, you will need to create a new payment token. Creating a new payment token does not require end user input.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_initiation_payment_token_create_request** | [**PaymentInitiationPaymentTokenCreateRequest**](PaymentInitiationPaymentTokenCreateRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentInitiationPaymentTokenCreateResponse**](PaymentInitiationPaymentTokenCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## credit_bank_income_get

> crate::models::CreditBankIncomeGetResponse credit_bank_income_get(credit_bank_income_get_request)
Retrieve information from the bank accounts used for income verification

`/credit/bank_income/get` returns the bank income report(s) for a specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credit_bank_income_get_request** | [**CreditBankIncomeGetRequest**](CreditBankIncomeGetRequest.md) |  | [required] |

### Return type

[**crate::models::CreditBankIncomeGetResponse**](CreditBankIncomeGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deposit_switch_alt_create

> crate::models::DepositSwitchAltCreateResponse deposit_switch_alt_create(deposit_switch_alt_create_request)
Create a deposit switch without using Plaid Exchange

This endpoint provides an alternative to `/deposit_switch/create` for customers who have not yet fully integrated with Plaid Exchange. Like `/deposit_switch/create`, it creates a deposit switch entity that will be persisted throughout the lifecycle of the switch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deposit_switch_alt_create_request** | [**DepositSwitchAltCreateRequest**](DepositSwitchAltCreateRequest.md) |  | [required] |

### Return type

[**crate::models::DepositSwitchAltCreateResponse**](DepositSwitchAltCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deposit_switch_create

> crate::models::DepositSwitchCreateResponse deposit_switch_create(deposit_switch_create_request)
Create a deposit switch

This endpoint creates a deposit switch entity that will be persisted throughout the lifecycle of the switch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deposit_switch_create_request** | [**DepositSwitchCreateRequest**](DepositSwitchCreateRequest.md) |  | [required] |

### Return type

[**crate::models::DepositSwitchCreateResponse**](DepositSwitchCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deposit_switch_get

> crate::models::DepositSwitchGetResponse deposit_switch_get(deposit_switch_get_request)
Retrieve a deposit switch

This endpoint returns information related to how the user has configured their payroll allocation and the state of the switch. You can use this information to build logic related to the user's direct deposit allocation preferences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deposit_switch_get_request** | [**DepositSwitchGetRequest**](DepositSwitchGetRequest.md) |  | [required] |

### Return type

[**crate::models::DepositSwitchGetResponse**](DepositSwitchGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deposit_switch_token_create

> crate::models::DepositSwitchTokenCreateResponse deposit_switch_token_create(deposit_switch_token_create_request)
Create a deposit switch token

In order for the end user to take action, you will need to create a public token representing the deposit switch. This token is used to initialize Link. It can be used one time and expires after 30 minutes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deposit_switch_token_create_request** | [**DepositSwitchTokenCreateRequest**](DepositSwitchTokenCreateRequest.md) |  | [required] |

### Return type

[**crate::models::DepositSwitchTokenCreateResponse**](DepositSwitchTokenCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## employers_search

> crate::models::EmployersSearchResponse employers_search(employers_search_request)
Search employer database

`/employers/search` allows you the ability to search Plaid’s database of known employers, for use with Deposit Switch. You can use this endpoint to look up a user's employer in order to confirm that they are supported. Users with non-supported employers can then be routed out of the Deposit Switch flow.  The data in the employer database is currently limited. As the Deposit Switch and Income products progress through their respective beta periods, more employers are being regularly added. Because the employer database is frequently updated, we recommend that you do not cache or store data from this endpoint for more than a day.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employers_search_request** | [**EmployersSearchRequest**](EmployersSearchRequest.md) |  | [required] |

### Return type

[**crate::models::EmployersSearchResponse**](EmployersSearchResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## employment_verification_get

> crate::models::EmploymentVerificationGetResponse employment_verification_get(employment_verification_get_request)
Retrieve a summary of an individual's employment information

`/employment/verification/get` returns a list of employments through a user payroll that was verified by an end user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employment_verification_get_request** | [**EmploymentVerificationGetRequest**](EmploymentVerificationGetRequest.md) |  | [required] |

### Return type

[**crate::models::EmploymentVerificationGetResponse**](EmploymentVerificationGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_get

> crate::models::IdentityGetResponse identity_get(identity_get_request)
Retrieve identity data

The `/identity/get` endpoint allows you to retrieve various account holder information on file with the financial institution, including names, emails, phone numbers, and addresses. Only name data is guaranteed to be returned; other fields will be empty arrays if not provided by the institution.  This request may take some time to complete if identity was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the data.  Note: In API versions 2018-05-22 and earlier, the `owners` object is not returned, and instead identity information is returned in the top level `identity` object. For more details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2019-05-29).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_get_request** | [**IdentityGetRequest**](IdentityGetRequest.md) |  | [required] |

### Return type

[**crate::models::IdentityGetResponse**](IdentityGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## income_verification_create

> crate::models::IncomeVerificationCreateResponse income_verification_create(income_verification_create_request)
(Deprecated) Create an income verification instance

`/income/verification/create` begins the income verification process by returning an `income_verification_id`. You can then provide the `income_verification_id` to `/link/token/create` under the `income_verification` parameter in order to create a Link instance that will prompt the user to go through the income verification flow. Plaid will fire an `INCOME` webhook once the user completes the Payroll Income flow, or when the uploaded documents in the Document Income flow have finished processing. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**income_verification_create_request** | [**IncomeVerificationCreateRequest**](IncomeVerificationCreateRequest.md) |  | [required] |

### Return type

[**crate::models::IncomeVerificationCreateResponse**](IncomeVerificationCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## income_verification_documents_download

> std::path::PathBuf income_verification_documents_download(income_verification_documents_download_request)
Download the original documents used for income verification

`/income/verification/documents/download` provides the ability to download the source documents associated with the verification.  If Document Income was used, the documents will be those the user provided in Link. For Payroll Income, the most recent files available for download from the payroll provider will be available from this endpoint.  The response to `/income/verification/documents/download` is a ZIP file in binary data. If a `document_id` is passed, a single document will be contained in this file. If not, the response will contain all documents associated with the verification.  The `request_id` is returned in the `Plaid-Request-ID` header.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**income_verification_documents_download_request** | [**IncomeVerificationDocumentsDownloadRequest**](IncomeVerificationDocumentsDownloadRequest.md) |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/zip

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## income_verification_paystubs_get

> crate::models::IncomeVerificationPaystubsGetResponse income_verification_paystubs_get(income_verification_paystubs_get_request)
Retrieve information from the paystubs used for income verification

`/income/verification/paystubs/get` returns the information collected from the paystubs that were used to verify an end user's income. It can be called once the status of the verification has been set to `VERIFICATION_STATUS_PROCESSING_COMPLETE`, as reported by the `INCOME: verification_status` webhook. Attempting to call the endpoint before verification has been completed will result in an error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**income_verification_paystubs_get_request** | [**IncomeVerificationPaystubsGetRequest**](IncomeVerificationPaystubsGetRequest.md) |  | [required] |

### Return type

[**crate::models::IncomeVerificationPaystubsGetResponse**](IncomeVerificationPaystubsGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## income_verification_precheck

> crate::models::IncomeVerificationPrecheckResponse income_verification_precheck(income_verification_precheck_request)
Check digital income verification eligibility and optimize conversion

`/income/verification/precheck` is an optional endpoint that can be called before initializing a Link session for income verification. It evaluates whether a given user is supportable by digital income verification and returns a `precheck_id` that can be provided to `/link/token/create`. If the user is eligible for digital verification, providing the `precheck_id` in this way will generate a Link UI optimized for the end user and their specific employer. If the user cannot be confirmed as eligible, the `precheck_id` can still be provided to `/link/token/create` and the user can still use the income verification flow, but they may be required to manually upload a paystub to verify their income.  While all request fields are optional, providing either `employer` or `transactions_access_tokens` data will increase the chance of receiving a useful result.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**income_verification_precheck_request** | [**IncomeVerificationPrecheckRequest**](IncomeVerificationPrecheckRequest.md) |  | [required] |

### Return type

[**crate::models::IncomeVerificationPrecheckResponse**](IncomeVerificationPrecheckResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## income_verification_refresh

> crate::models::IncomeVerificationRefreshResponse income_verification_refresh(income_verification_refresh_request)
Refresh an income verification

`/income/verification/refresh` refreshes a given income verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**income_verification_refresh_request** | [**IncomeVerificationRefreshRequest**](IncomeVerificationRefreshRequest.md) |  | [required] |

### Return type

[**crate::models::IncomeVerificationRefreshResponse**](IncomeVerificationRefreshResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## income_verification_taxforms_get

> crate::models::IncomeVerificationTaxformsGetResponse income_verification_taxforms_get(request_body)
Retrieve information from the tax documents used for income verification

`/income/verification/taxforms/get` returns the information collected from forms that were used to verify an end user's income. It can be called once the status of the verification has been set to `VERIFICATION_STATUS_PROCESSING_COMPLETE`, as reported by the `INCOME: verification_status` webhook. Attempting to call the endpoint before verification has been completed will result in an error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [required] |

### Return type

[**crate::models::IncomeVerificationTaxformsGetResponse**](IncomeVerificationTaxformsGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## institutions_get

> crate::models::InstitutionsGetResponse institutions_get(institutions_get_request)
Get details of all supported institutions

Returns a JSON response containing details on all financial institutions currently supported by Plaid. Because Plaid supports thousands of institutions, results are paginated.  If there is no overlap between an institution’s enabled products and a client’s enabled products, then the institution will be filtered out from the response. As a result, the number of institutions returned may not match the count specified in the call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**institutions_get_request** | [**InstitutionsGetRequest**](InstitutionsGetRequest.md) |  | [required] |

### Return type

[**crate::models::InstitutionsGetResponse**](InstitutionsGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## institutions_get_by_id

> crate::models::InstitutionsGetByIdResponse institutions_get_by_id(institutions_get_by_id_request)
Get details of an institution

Returns a JSON response containing details on a specified financial institution currently supported by Plaid.  Versioning note: API versions 2019-05-29 and earlier allow use of the `public_key` parameter instead of the `client_id` and `secret` to authenticate to this endpoint. The `public_key` has been deprecated; all customers are encouraged to use `client_id` and `secret` instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**institutions_get_by_id_request** | [**InstitutionsGetByIdRequest**](InstitutionsGetByIdRequest.md) |  | [required] |

### Return type

[**crate::models::InstitutionsGetByIdResponse**](InstitutionsGetByIdResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## institutions_search

> crate::models::InstitutionsSearchResponse institutions_search(institutions_search_request)
Search institutions

Returns a JSON response containing details for institutions that match the query parameters, up to a maximum of ten institutions per query.  Versioning note: API versions 2019-05-29 and earlier allow use of the `public_key` parameter instead of the `client_id` and `secret` parameters to authenticate to this endpoint. The `public_key` parameter has since been deprecated; all customers are encouraged to use `client_id` and `secret` instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**institutions_search_request** | [**InstitutionsSearchRequest**](InstitutionsSearchRequest.md) |  | [required] |

### Return type

[**crate::models::InstitutionsSearchResponse**](InstitutionsSearchResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## investments_holdings_get

> crate::models::InvestmentsHoldingsGetResponse investments_holdings_get(investments_holdings_get_request)
Get Investment holdings

The `/investments/holdings/get` endpoint allows developers to receive user-authorized stock position data for `investment`-type accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**investments_holdings_get_request** | [**InvestmentsHoldingsGetRequest**](InvestmentsHoldingsGetRequest.md) |  | [required] |

### Return type

[**crate::models::InvestmentsHoldingsGetResponse**](InvestmentsHoldingsGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## investments_transactions_get

> crate::models::InvestmentsTransactionsGetResponse investments_transactions_get(investments_transactions_get_request)
Get investment transactions

The `/investments/transactions/get` endpoint allows developers to retrieve user-authorized transaction data for investment accounts.  Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift.  Due to the potentially large number of investment transactions associated with an Item, results are paginated. Manipulate the count and offset parameters in conjunction with the `total_investment_transactions` response body field to fetch all available investment transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**investments_transactions_get_request** | [**InvestmentsTransactionsGetRequest**](InvestmentsTransactionsGetRequest.md) |  | [required] |

### Return type

[**crate::models::InvestmentsTransactionsGetResponse**](InvestmentsTransactionsGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## item_access_token_invalidate

> crate::models::ItemAccessTokenInvalidateResponse item_access_token_invalidate(item_access_token_invalidate_request)
Invalidate access_token

By default, the `access_token` associated with an Item does not expire and should be stored in a persistent, secure manner.  You can use the `/item/access_token/invalidate` endpoint to rotate the `access_token` associated with an Item. The endpoint returns a new `access_token` and immediately invalidates the previous `access_token`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_access_token_invalidate_request** | [**ItemAccessTokenInvalidateRequest**](ItemAccessTokenInvalidateRequest.md) |  | [required] |

### Return type

[**crate::models::ItemAccessTokenInvalidateResponse**](ItemAccessTokenInvalidateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## item_application_list

> crate::models::ItemApplicationListResponse item_application_list(item_application_list_request)
List a user’s connected applications

List a user’s connected applications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_application_list_request** | [**ItemApplicationListRequest**](ItemApplicationListRequest.md) |  | [required] |

### Return type

[**crate::models::ItemApplicationListResponse**](ItemApplicationListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## item_application_scopes_update

> crate::models::ItemApplicationScopesUpdateResponse item_application_scopes_update(item_application_scopes_update_request)
Update the scopes of access for a particular application

Enable consumers to update product access on selected accounts for an application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_application_scopes_update_request** | [**ItemApplicationScopesUpdateRequest**](ItemApplicationScopesUpdateRequest.md) |  | [required] |

### Return type

[**crate::models::ItemApplicationScopesUpdateResponse**](ItemApplicationScopesUpdateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## item_create_public_token

> crate::models::ItemPublicTokenCreateResponse item_create_public_token(item_public_token_create_request)
Create public token

Note: As of July 2020, the `/item/public_token/create` endpoint is deprecated. Instead, use `/link/token/create` with an `access_token` to create a Link token for use with [update mode](https://plaid.com/docs/link/update-mode).  If you need your user to take action to restore or resolve an error associated with an Item, generate a public token with the `/item/public_token/create` endpoint and then initialize Link with that `public_token`.  A `public_token` is one-time use and expires after 30 minutes. You use a `public_token` to initialize Link in [update mode](https://plaid.com/docs/link/update-mode) for a particular Item. You can generate a `public_token` for an Item even if you did not use Link to create the Item originally.  The `/item/public_token/create` endpoint is **not** used to create your initial `public_token`. If you have not already received an `access_token` for a specific Item, use Link to obtain your `public_token` instead. See the [Quickstart](https://plaid.com/docs/quickstart) for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_public_token_create_request** | [**ItemPublicTokenCreateRequest**](ItemPublicTokenCreateRequest.md) |  | [required] |

### Return type

[**crate::models::ItemPublicTokenCreateResponse**](ItemPublicTokenCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## item_get

> crate::models::ItemGetResponse item_get(item_get_request)
Retrieve an Item

Returns information about the status of an Item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_get_request** | [**ItemGetRequest**](ItemGetRequest.md) |  | [required] |

### Return type

[**crate::models::ItemGetResponse**](ItemGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## item_import

> crate::models::ItemImportResponse item_import(item_import_request)
Import Item

`/item/import` creates an Item via your Plaid Exchange Integration and returns an `access_token`. As part of an `/item/import` request, you will include a User ID (`user_auth.user_id`) and Authentication Token (`user_auth.auth_token`) that enable data aggregation through your Plaid Exchange API endpoints. These authentication principals are to be chosen by you.  Upon creating an Item via `/item/import`, Plaid will automatically begin an extraction of that Item through the Plaid Exchange infrastructure you have already integrated. This will automatically generate the Plaid native account ID for the account the user will switch their direct deposit to (`target_account_id`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_import_request** | [**ItemImportRequest**](ItemImportRequest.md) |  | [required] |

### Return type

[**crate::models::ItemImportResponse**](ItemImportResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## item_public_token_exchange

> crate::models::ItemPublicTokenExchangeResponse item_public_token_exchange(item_public_token_exchange_request)
Exchange public token for an access token

Exchange a Link `public_token` for an API `access_token`. Link hands off the `public_token` client-side via the `onSuccess` callback once a user has successfully created an Item. The `public_token` is ephemeral and expires after 30 minutes. An `access_token` does not expire, but can be revoked by calling `/item/remove`.  The response also includes an `item_id` that should be stored with the `access_token`. The `item_id` is used to identify an Item in a webhook. The `item_id` can also be retrieved by making an `/item/get` request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_public_token_exchange_request** | [**ItemPublicTokenExchangeRequest**](ItemPublicTokenExchangeRequest.md) |  | [required] |

### Return type

[**crate::models::ItemPublicTokenExchangeResponse**](ItemPublicTokenExchangeResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## item_remove

> crate::models::ItemRemoveResponse item_remove(item_remove_request)
Remove an Item

The `/item/remove` endpoint allows you to remove an Item. Once removed, the `access_token`, as well as any processor tokens or bank account tokens associated with the Item, is no longer valid and cannot be used to access any data that was associated with the Item.  Note that in the Development environment, issuing an `/item/remove`  request will not decrement your live credential count. To increase your credential account in Development, contact Support.  Also note that for certain OAuth-based institutions, an Item removed via `/item/remove` may still show as an active connection in the institution's OAuth permission manager.  API versions 2019-05-29 and earlier return a `removed` boolean as part of the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_remove_request** | [**ItemRemoveRequest**](ItemRemoveRequest.md) |  | [required] |

### Return type

[**crate::models::ItemRemoveResponse**](ItemRemoveResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## item_webhook_update

> crate::models::ItemWebhookUpdateResponse item_webhook_update(item_webhook_update_request)
Update Webhook URL

The POST `/item/webhook/update` allows you to update the webhook URL associated with an Item. This request triggers a [`WEBHOOK_UPDATE_ACKNOWLEDGED`](https://plaid.com/docs/api/items/#webhook_update_acknowledged) webhook to the newly specified webhook URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_webhook_update_request** | [**ItemWebhookUpdateRequest**](ItemWebhookUpdateRequest.md) |  | [required] |

### Return type

[**crate::models::ItemWebhookUpdateResponse**](ItemWebhookUpdateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## liabilities_get

> crate::models::LiabilitiesGetResponse liabilities_get(liabilities_get_request)
Retrieve Liabilities data

The `/liabilities/get` endpoint returns various details about an Item with loan or credit accounts. Liabilities data is available primarily for US financial institutions, with some limited coverage of Canadian institutions. Currently supported account types are account type `credit` with account subtype `credit card` or `paypal`, and account type `loan` with account subtype `student` or `mortgage`. To limit accounts listed in Link to types and subtypes supported by Liabilities, you can use the `account_filters` parameter when [creating a Link token](https://plaid.com/docs/api/tokens/#linktokencreate).  The types of information returned by Liabilities can include balances and due dates, loan terms, and account details such as original loan amount and guarantor. Data is refreshed approximately once per day; the latest data can be retrieved by calling `/liabilities/get`.  Note: This request may take some time to complete if `liabilities` was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the additional data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**liabilities_get_request** | [**LiabilitiesGetRequest**](LiabilitiesGetRequest.md) |  | [required] |

### Return type

[**crate::models::LiabilitiesGetResponse**](LiabilitiesGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_token_create

> crate::models::LinkTokenCreateResponse link_token_create(link_token_create_request)
Create Link Token

The `/link/token/create` endpoint creates a `link_token`, which is required as a parameter when initializing Link. Once Link has been initialized, it returns a `public_token`, which can then be exchanged for an `access_token` via `/item/public_token/exchange` as part of the main Link flow.  A `link_token` generated by `/link/token/create` is also used to initialize other Link flows, such as the update mode flow for tokens with expired credentials, or the Payment Initiation (Europe) flow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_token_create_request** | [**LinkTokenCreateRequest**](LinkTokenCreateRequest.md) |  | [required] |

### Return type

[**crate::models::LinkTokenCreateResponse**](LinkTokenCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_token_get

> crate::models::LinkTokenGetResponse link_token_get(link_token_get_request)
Get Link Token

The `/link/token/get` endpoint gets information about a previously-created `link_token` using the `/link/token/create` endpoint. It can be useful for debugging purposes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_token_get_request** | [**LinkTokenGetRequest**](LinkTokenGetRequest.md) |  | [required] |

### Return type

[**crate::models::LinkTokenGetResponse**](LinkTokenGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_initiation_consent_create

> crate::models::PaymentInitiationConsentCreateResponse payment_initiation_consent_create(payment_initiation_consent_create_request)
Create payment consent

The `/payment_initiation/consent/create` endpoint is used to create a payment consent, which can be used to initiate payments on behalf of the user. Payment consents are created with `UNAUTHORISED` status by default and must be authorised by the user before payments can be initiated.  Consents can be limited in time and scope, and have constraints that describe limitations for payments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_initiation_consent_create_request** | [**PaymentInitiationConsentCreateRequest**](PaymentInitiationConsentCreateRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentInitiationConsentCreateResponse**](PaymentInitiationConsentCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_initiation_consent_get

> crate::models::PaymentInitiationConsentGetResponse payment_initiation_consent_get(payment_initiation_consent_get_request)
Get payment consent

The `/payment_initiation/consent/get` endpoint can be used to check the status of a payment consent, as well as to receive basic information such as recipient and constraints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_initiation_consent_get_request** | [**PaymentInitiationConsentGetRequest**](PaymentInitiationConsentGetRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentInitiationConsentGetResponse**](PaymentInitiationConsentGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_initiation_consent_payment_execute

> crate::models::PaymentInitiationConsentPaymentExecuteResponse payment_initiation_consent_payment_execute(payment_initiation_consent_payment_execute_request)
Execute a single payment using consent

The `/payment_initiation/consent/payment/execute` endpoint can be used to execute payments using payment consent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_initiation_consent_payment_execute_request** | [**PaymentInitiationConsentPaymentExecuteRequest**](PaymentInitiationConsentPaymentExecuteRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentInitiationConsentPaymentExecuteResponse**](PaymentInitiationConsentPaymentExecuteResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_initiation_consent_revoke

> crate::models::PaymentInitiationConsentRevokeResponse payment_initiation_consent_revoke(payment_initiation_consent_revoke_request)
Revoke payment consent

The `/payment_initiation/consent/revoke` endpoint can be used to revoke the payment consent. Once the consent is revoked, it is not possible to initiate payments using it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_initiation_consent_revoke_request** | [**PaymentInitiationConsentRevokeRequest**](PaymentInitiationConsentRevokeRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentInitiationConsentRevokeResponse**](PaymentInitiationConsentRevokeResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_initiation_payment_create

> crate::models::PaymentInitiationPaymentCreateResponse payment_initiation_payment_create(payment_initiation_payment_create_request)
Create a payment

After creating a payment recipient, you can use the `/payment_initiation/payment/create` endpoint to create a payment to that recipient.  Payments can be one-time or standing order (recurring) and can be denominated in either EUR or GBP.  If making domestic GBP-denominated payments, your recipient must have been created with BACS numbers. In general, EUR-denominated payments will be sent via SEPA Credit Transfer and GBP-denominated payments will be sent via the Faster Payments network, but the payment network used will be determined by the institution. Payments sent via Faster Payments will typically arrive immediately, while payments sent via SEPA Credit Transfer will typically arrive in one business day.  Standing orders (recurring payments) must be denominated in GBP and can only be sent to recipients in the UK. Once created, standing order payments cannot be modified or canceled via the API. An end user can cancel or modify a standing order directly on their banking application or website, or by contacting the bank. Standing orders will follow the payment rules of the underlying rails (Faster Payments in UK). Payments can be sent Monday to Friday, excluding bank holidays. If the pre-arranged date falls on a weekend or bank holiday, the payment is made on the next working day. It is not possible to guarantee the exact time the payment will reach the recipient’s account, although at least 90% of standing order payments are sent by 6am.  In the Development environment, payments must be below 5 GBP / EUR. For details on any payment limits in Production, contact your Plaid Account Manager.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_initiation_payment_create_request** | [**PaymentInitiationPaymentCreateRequest**](PaymentInitiationPaymentCreateRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentInitiationPaymentCreateResponse**](PaymentInitiationPaymentCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_initiation_payment_get

> crate::models::PaymentInitiationPaymentGetResponse payment_initiation_payment_get(payment_initiation_payment_get_request)
Get payment details

The `/payment_initiation/payment/get` endpoint can be used to check the status of a payment, as well as to receive basic information such as recipient and payment amount. In the case of standing orders, the `/payment_initiation/payment/get` endpoint will provide information about the status of the overall standing order itself; the API cannot be used to retrieve payment status for individual payments within a standing order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_initiation_payment_get_request** | [**PaymentInitiationPaymentGetRequest**](PaymentInitiationPaymentGetRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentInitiationPaymentGetResponse**](PaymentInitiationPaymentGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_initiation_payment_list

> crate::models::PaymentInitiationPaymentListResponse payment_initiation_payment_list(payment_initiation_payment_list_request)
List payments

The `/payment_initiation/payment/list` endpoint can be used to retrieve all created payments. By default, the 10 most recent payments are returned. You can request more payments and paginate through the results using the optional `count` and `cursor` parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_initiation_payment_list_request** | [**PaymentInitiationPaymentListRequest**](PaymentInitiationPaymentListRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentInitiationPaymentListResponse**](PaymentInitiationPaymentListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_initiation_payment_reverse

> crate::models::PaymentInitiationPaymentReverseResponse payment_initiation_payment_reverse(payment_initiation_payment_reverse_request)
Reverse an existing payment

Reverse a previously initiated payment.  A payment can only be reversed once and will be refunded to the original sender's account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_initiation_payment_reverse_request** | [**PaymentInitiationPaymentReverseRequest**](PaymentInitiationPaymentReverseRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentInitiationPaymentReverseResponse**](PaymentInitiationPaymentReverseResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_initiation_recipient_create

> crate::models::PaymentInitiationRecipientCreateResponse payment_initiation_recipient_create(payment_initiation_recipient_create_request)
Create payment recipient

Create a payment recipient for payment initiation.  The recipient must be in Europe, within a country that is a member of the Single Euro Payment Area (SEPA).  For a standing order (recurring) payment, the recipient must be in the UK.  The endpoint is idempotent: if a developer has already made a request with the same payment details, Plaid will return the same `recipient_id`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_initiation_recipient_create_request** | [**PaymentInitiationRecipientCreateRequest**](PaymentInitiationRecipientCreateRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentInitiationRecipientCreateResponse**](PaymentInitiationRecipientCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_initiation_recipient_get

> crate::models::PaymentInitiationRecipientGetResponse payment_initiation_recipient_get(payment_initiation_recipient_get_request)
Get payment recipient

Get details about a payment recipient you have previously created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_initiation_recipient_get_request** | [**PaymentInitiationRecipientGetRequest**](PaymentInitiationRecipientGetRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentInitiationRecipientGetResponse**](PaymentInitiationRecipientGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_initiation_recipient_list

> crate::models::PaymentInitiationRecipientListResponse payment_initiation_recipient_list(payment_initiation_recipient_list_request)
List payment recipients

The `/payment_initiation/recipient/list` endpoint list the payment recipients that you have previously created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_initiation_recipient_list_request** | [**PaymentInitiationRecipientListRequest**](PaymentInitiationRecipientListRequest.md) |  | [required] |

### Return type

[**crate::models::PaymentInitiationRecipientListResponse**](PaymentInitiationRecipientListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## processor_apex_processor_token_create

> crate::models::ProcessorTokenCreateResponse processor_apex_processor_token_create(processor_apex_processor_token_create_request)
Create Apex bank account token

Used to create a token suitable for sending to Apex to enable Plaid-Apex integrations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**processor_apex_processor_token_create_request** | [**ProcessorApexProcessorTokenCreateRequest**](ProcessorApexProcessorTokenCreateRequest.md) |  | [required] |

### Return type

[**crate::models::ProcessorTokenCreateResponse**](ProcessorTokenCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## processor_auth_get

> crate::models::ProcessorAuthGetResponse processor_auth_get(processor_auth_get_request)
Retrieve Auth data

The `/processor/auth/get` endpoint returns the bank account and bank identification number (such as the routing number, for US accounts), for a checking or savings account that''s associated with a given `processor_token`. The endpoint also returns high-level account data and balances when available.  Versioning note: API versions 2019-05-29 and earlier use a different schema for the `numbers` object returned by this endpoint. For details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2020-09-14). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**processor_auth_get_request** | [**ProcessorAuthGetRequest**](ProcessorAuthGetRequest.md) |  | [required] |

### Return type

[**crate::models::ProcessorAuthGetResponse**](ProcessorAuthGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## processor_balance_get

> crate::models::ProcessorBalanceGetResponse processor_balance_get(processor_balance_get_request)
Retrieve Balance data

The `/processor/balance/get` endpoint returns the real-time balance for each of an Item's accounts. While other endpoints may return a balance object, only `/processor/balance/get` forces the available and current balance fields to be refreshed rather than cached. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**processor_balance_get_request** | [**ProcessorBalanceGetRequest**](ProcessorBalanceGetRequest.md) | The `/processor/balance/get` endpoint returns the real-time balance for the account associated with a given `processor_token`.  The current balance is the total amount of funds in the account. The available balance is the current balance less any outstanding holds or debits that have not yet posted to the account.  Note that not all institutions calculate the available balance. In the event that available balance is unavailable from the institution, Plaid will return an available balance value of `null`. | [required] |

### Return type

[**crate::models::ProcessorBalanceGetResponse**](ProcessorBalanceGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## processor_bank_transfer_create

> crate::models::ProcessorBankTransferCreateResponse processor_bank_transfer_create(processor_bank_transfer_create_request)
Create a bank transfer as a processor

Use the `/processor/bank_transfer/create` endpoint to initiate a new bank transfer as a processor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**processor_bank_transfer_create_request** | [**ProcessorBankTransferCreateRequest**](ProcessorBankTransferCreateRequest.md) |  | [required] |

### Return type

[**crate::models::ProcessorBankTransferCreateResponse**](ProcessorBankTransferCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## processor_identity_get

> crate::models::ProcessorIdentityGetResponse processor_identity_get(processor_identity_get_request)
Retrieve Identity data

The `/processor/identity/get` endpoint allows you to retrieve various account holder information on file with the financial institution, including names, emails, phone numbers, and addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**processor_identity_get_request** | [**ProcessorIdentityGetRequest**](ProcessorIdentityGetRequest.md) |  | [required] |

### Return type

[**crate::models::ProcessorIdentityGetResponse**](ProcessorIdentityGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## processor_stripe_bank_account_token_create

> crate::models::ProcessorStripeBankAccountTokenCreateResponse processor_stripe_bank_account_token_create(processor_stripe_bank_account_token_create_request)
Create Stripe bank account token

Used to create a token suitable for sending to Stripe to enable Plaid-Stripe integrations. For a detailed guide on integrating Stripe, see [Add Stripe to your app](https://plaid.com/docs/auth/partnerships/stripe/). Bank account tokens can also be revoked, using `/item/remove`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**processor_stripe_bank_account_token_create_request** | [**ProcessorStripeBankAccountTokenCreateRequest**](ProcessorStripeBankAccountTokenCreateRequest.md) |  | [required] |

### Return type

[**crate::models::ProcessorStripeBankAccountTokenCreateResponse**](ProcessorStripeBankAccountTokenCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## processor_token_create

> crate::models::ProcessorTokenCreateResponse processor_token_create(processor_token_create_request)
Create processor token

Used to create a token suitable for sending to one of Plaid's partners to enable integrations. Note that Stripe partnerships use bank account tokens instead; see `/processor/stripe/bank_account_token/create` for creating tokens for use with Stripe integrations. Processor tokens can also be revoked, using `/item/remove`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**processor_token_create_request** | [**ProcessorTokenCreateRequest**](ProcessorTokenCreateRequest.md) |  | [required] |

### Return type

[**crate::models::ProcessorTokenCreateResponse**](ProcessorTokenCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_bank_transfer_fire_webhook

> crate::models::SandboxBankTransferFireWebhookResponse sandbox_bank_transfer_fire_webhook(sandbox_bank_transfer_fire_webhook_request)
Manually fire a Bank Transfer webhook

Use the `/sandbox/bank_transfer/fire_webhook` endpoint to manually trigger a Bank Transfers webhook in the Sandbox environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_bank_transfer_fire_webhook_request** | [**SandboxBankTransferFireWebhookRequest**](SandboxBankTransferFireWebhookRequest.md) |  | [required] |

### Return type

[**crate::models::SandboxBankTransferFireWebhookResponse**](SandboxBankTransferFireWebhookResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_bank_transfer_simulate

> crate::models::SandboxBankTransferSimulateResponse sandbox_bank_transfer_simulate(sandbox_bank_transfer_simulate_request)
Simulate a bank transfer event in Sandbox

Use the `/sandbox/bank_transfer/simulate` endpoint to simulate a bank transfer event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/bank_transfer/event/sync` or `/bank_transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_bank_transfer_simulate_request** | [**SandboxBankTransferSimulateRequest**](SandboxBankTransferSimulateRequest.md) |  | [required] |

### Return type

[**crate::models::SandboxBankTransferSimulateResponse**](SandboxBankTransferSimulateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_income_fire_webhook

> crate::models::SandboxIncomeFireWebhookResponse sandbox_income_fire_webhook(sandbox_income_fire_webhook_request)
Manually fire an Income webhook

Use the `/sandbox/income/fire_webhook` endpoint to manually trigger an Income webhook in the Sandbox environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_income_fire_webhook_request** | [**SandboxIncomeFireWebhookRequest**](SandboxIncomeFireWebhookRequest.md) |  | [required] |

### Return type

[**crate::models::SandboxIncomeFireWebhookResponse**](SandboxIncomeFireWebhookResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_item_fire_webhook

> crate::models::SandboxItemFireWebhookResponse sandbox_item_fire_webhook(sandbox_item_fire_webhook_request)
Fire a test webhook

The `/sandbox/item/fire_webhook` endpoint is used to test that code correctly handles webhooks. This endpoint can trigger a Transactions `DEFAULT_UPDATE` webhook to be fired for a given Sandbox Item. If the Item does not support Transactions, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result. This endpoint can also trigger a `NEW_ACCOUNTS_AVAILABLE` webhook to be fired for a given Sandbox Item created with Account Select v2. Note that this endpoint is provided for developer ease-of-use and is not required for testing webhooks; webhooks will also fire in Sandbox under the same conditions that they would in Production or Development.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_item_fire_webhook_request** | [**SandboxItemFireWebhookRequest**](SandboxItemFireWebhookRequest.md) |  | [required] |

### Return type

[**crate::models::SandboxItemFireWebhookResponse**](SandboxItemFireWebhookResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_item_reset_login

> crate::models::SandboxItemResetLoginResponse sandbox_item_reset_login(sandbox_item_reset_login_request)
Force a Sandbox Item into an error state

`/sandbox/item/reset_login/` forces an Item into an `ITEM_LOGIN_REQUIRED` state in order to simulate an Item whose login is no longer valid. This makes it easy to test Link's [update mode](https://plaid.com/docs/link/update-mode) flow in the Sandbox environment.  After calling `/sandbox/item/reset_login`, You can then use Plaid Link update mode to restore the Item to a good state. An `ITEM_LOGIN_REQUIRED` webhook will also be fired after a call to this endpoint, if one is associated with the Item.   In the Sandbox, Items will transition to an `ITEM_LOGIN_REQUIRED` error state automatically after 30 days, even if this endpoint is not called.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_item_reset_login_request** | [**SandboxItemResetLoginRequest**](SandboxItemResetLoginRequest.md) |  | [required] |

### Return type

[**crate::models::SandboxItemResetLoginResponse**](SandboxItemResetLoginResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_item_set_verification_status

> crate::models::SandboxItemSetVerificationStatusResponse sandbox_item_set_verification_status(sandbox_item_set_verification_status_request)
Set verification status for Sandbox account

The `/sandbox/item/set_verification_status` endpoint can be used to change the verification status of an Item in in the Sandbox in order to simulate the Automated Micro-deposit flow.  Note that not all Plaid developer accounts are enabled for micro-deposit based verification by default. Your account must be enabled for this feature in order to test it in Sandbox. To enable this features or check your status, contact your account manager or [submit a product access Support ticket](https://dashboard.plaid.com/support/new/product-and-development/product-troubleshooting/request-product-access).  For more information on testing Automated Micro-deposits in Sandbox, see [Auth full coverage testing](https://plaid.com/docs/auth/coverage/testing#).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_item_set_verification_status_request** | [**SandboxItemSetVerificationStatusRequest**](SandboxItemSetVerificationStatusRequest.md) |  | [required] |

### Return type

[**crate::models::SandboxItemSetVerificationStatusResponse**](SandboxItemSetVerificationStatusResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_oauth_select_accounts

> ::std::collections::HashMap<String, serde_json::Value> sandbox_oauth_select_accounts(sandbox_oauth_select_accounts_request)
Save the selected accounts when connecting to the Platypus Oauth institution

Save the selected accounts when connecting to the Platypus Oauth institution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_oauth_select_accounts_request** | [**SandboxOauthSelectAccountsRequest**](SandboxOauthSelectAccountsRequest.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_processor_token_create

> crate::models::SandboxProcessorTokenCreateResponse sandbox_processor_token_create(sandbox_processor_token_create_request)
Create a test Item and processor token

Use the `/sandbox/processor_token/create` endpoint to create a valid `processor_token` for an arbitrary institution ID and test credentials. The created `processor_token` corresponds to a new Sandbox Item. You can then use this `processor_token` with the `/processor/` API endpoints in Sandbox. You can also use `/sandbox/processor_token/create` with the [`user_custom` test username](https://plaid.com/docs/sandbox/user-custom) to generate a test account with custom data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_processor_token_create_request** | [**SandboxProcessorTokenCreateRequest**](SandboxProcessorTokenCreateRequest.md) |  | [required] |

### Return type

[**crate::models::SandboxProcessorTokenCreateResponse**](SandboxProcessorTokenCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_public_token_create

> crate::models::SandboxPublicTokenCreateResponse sandbox_public_token_create(sandbox_public_token_create_request)
Create a test Item

Use the `/sandbox/public_token/create` endpoint to create a valid `public_token`  for an arbitrary institution ID, initial products, and test credentials. The created `public_token` maps to a new Sandbox Item. You can then call `/item/public_token/exchange` to exchange the `public_token` for an `access_token` and perform all API actions. `/sandbox/public_token/create` can also be used with the [`user_custom` test username](https://plaid.com/docs/sandbox/user-custom) to generate a test account with custom data. `/sandbox/public_token/create` cannot be used with OAuth institutions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_public_token_create_request** | [**SandboxPublicTokenCreateRequest**](SandboxPublicTokenCreateRequest.md) |  | [required] |

### Return type

[**crate::models::SandboxPublicTokenCreateResponse**](SandboxPublicTokenCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_transfer_repayment_simulate

> crate::models::SandboxTransferRepaymentSimulateResponse sandbox_transfer_repayment_simulate(sandbox_transfer_repayment_simulate_request)
Trigger the creation of a repayment

Use the `/sandbox/transfer/repayment/simulate` endpoint to trigger the creation of a repayment. As a side effect of calling this route, a repayment is created that includes all unreimbursed returns of guaranteed transfers. If there are no such returns, an 400 error is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_transfer_repayment_simulate_request** | [**SandboxTransferRepaymentSimulateRequest**](SandboxTransferRepaymentSimulateRequest.md) |  | [required] |

### Return type

[**crate::models::SandboxTransferRepaymentSimulateResponse**](SandboxTransferRepaymentSimulateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_transfer_simulate

> crate::models::SandboxTransferSimulateResponse sandbox_transfer_simulate(sandbox_transfer_simulate_request)
Simulate a transfer event in Sandbox

Use the `/sandbox/transfer/simulate` endpoint to simulate a transfer event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/transfer/event/sync` or `/transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_transfer_simulate_request** | [**SandboxTransferSimulateRequest**](SandboxTransferSimulateRequest.md) |  | [required] |

### Return type

[**crate::models::SandboxTransferSimulateResponse**](SandboxTransferSimulateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sandbox_transfer_sweep_simulate

> crate::models::SandboxTransferSweepSimulateResponse sandbox_transfer_sweep_simulate(sandbox_transfer_sweep_simulate_request)
Simulate creating a sweep

Use the `/sandbox/transfer/sweep/simulate` endpoint to create a sweep and associated events in the Sandbox environment. Upon calling this endpoint, all `posted` or `pending` transfers with a sweep status of `unswept` will become `swept`, and all `reversed` transfers with a sweep status of `swept` will become `reverse_swept`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_transfer_sweep_simulate_request** | [**SandboxTransferSweepSimulateRequest**](SandboxTransferSweepSimulateRequest.md) |  | [required] |

### Return type

[**crate::models::SandboxTransferSweepSimulateResponse**](SandboxTransferSweepSimulateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## signal_decision_report

> crate::models::SignalDecisionReportResponse signal_decision_report(signal_decision_report_request)
Report whether you initiated an ACH transaction

After calling `/signal/evaluate`, call `/signal/decision/report` to report whether the transaction was initiated. This endpoint will return an `INVALID_REQUEST` error if called a second time with a different value for `initiated`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signal_decision_report_request** | [**SignalDecisionReportRequest**](SignalDecisionReportRequest.md) |  | [required] |

### Return type

[**crate::models::SignalDecisionReportResponse**](SignalDecisionReportResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## signal_evaluate

> crate::models::SignalEvaluateResponse signal_evaluate(signal_evaluate_request)
Evaluate a planned ACH transaction

Use `/signal/evaluate` to evaluate a planned ACH transaction to get a return risk assessment (such as a risk score and risk tier) and additional risk signals.  In order to obtain a valid score for an ACH transaction, Plaid must have an access token for the account, and the Item must be healthy (receiving product updates) or have recently been in a healthy state. If the transaction does not meet eligibility requirements, an error will be returned corresponding to the underlying cause. If `/signal/evaluate` is called on the same transaction multiple times within a 24-hour period, cached results may be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signal_evaluate_request** | [**SignalEvaluateRequest**](SignalEvaluateRequest.md) |  | [required] |

### Return type

[**crate::models::SignalEvaluateResponse**](SignalEvaluateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## signal_return_report

> crate::models::SignalReturnReportResponse signal_return_report(signal_return_report_request)
Report a return for an ACH transaction

Call the `/signal/return/report` endpoint to report a returned transaction that was previously sent to the `/signal/evaluate` endpoint. Your feedback will be used by the model to incorporate the latest risk trend in your portfolio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signal_return_report_request** | [**SignalReturnReportRequest**](SignalReturnReportRequest.md) |  | [required] |

### Return type

[**crate::models::SignalReturnReportResponse**](SignalReturnReportResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_enhance

> crate::models::TransactionsEnhanceGetResponse transactions_enhance(transactions_enhance_get_request)
enhance locally-held transaction data

The '/beta/transactions/v1/enhance' endpoint enriches raw transaction data provided directly by clients.  The product is currently in beta.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transactions_enhance_get_request** | [**TransactionsEnhanceGetRequest**](TransactionsEnhanceGetRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionsEnhanceGetResponse**](TransactionsEnhanceGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_get

> crate::models::TransactionsGetResponse transactions_get(transactions_get_request)
Get transaction data

The `/transactions/get` endpoint allows developers to receive user-authorized transaction data for credit, depository, and some loan-type accounts (only those with account subtype `student`; coverage may be limited). For transaction history from investments accounts, use the [Investments endpoint](https://plaid.com/docs/api/products/investments/) instead. Transaction data is standardized across financial institutions, and in many cases transactions are linked to a clean name, entity type, location, and category. Similarly, account data is standardized and returned with a clean name, number, balance, and other meta information where available.  Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift.  Transactions are not immutable and can also be removed altogether by the institution; a removed transaction will no longer appear in `/transactions/get`.  For more details, see [Pending and posted transactions](https://plaid.com/docs/transactions/transactions-data/#pending-and-posted-transactions).  Due to the potentially large number of transactions associated with an Item, results are paginated. Manipulate the `count` and `offset` parameters in conjunction with the `total_transactions` response body field to fetch all available transactions.  Data returned by `/transactions/get` will be the data available for the Item as of the most recent successful check for new transactions. Plaid typically checks for new data multiple times a day, but these checks may occur less frequently, such as once a day, depending on the institution. An Item's `status.transactions.last_successful_update` field will show the timestamp of the most recent successful update. To force Plaid to check for new transactions, you can use the `/transactions/refresh` endpoint.  Note that data may not be immediately available to `/transactions/get`. Plaid will begin to prepare transactions data upon Item link, if Link was initialized with `transactions`, or upon the first call to `/transactions/get`, if it wasn't. To be alerted when transaction data is ready to be fetched, listen for the [`INITIAL_UPDATE`](https://plaid.com/docs/api/products/transactions/#initial_update) and [`HISTORICAL_UPDATE`](https://plaid.com/docs/api/products/transactions/#historical_update) webhooks. If no transaction history is ready when `/transactions/get` is called, it will return a `PRODUCT_NOT_READY` error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transactions_get_request** | [**TransactionsGetRequest**](TransactionsGetRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionsGetResponse**](TransactionsGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_recurring_get

> crate::models::TransactionsRecurringGetResponse transactions_recurring_get(transactions_recurring_get_request)
Get streams of recurring transactions

The `/transactions/recurring/get` endpoint identifies and returns groups of transactions that occur on a regular basis for the inputted Item and accounts.  The product is currently in beta. To request access, contact transactions-feedback@plaid.com.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transactions_recurring_get_request** | [**TransactionsRecurringGetRequest**](TransactionsRecurringGetRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionsRecurringGetResponse**](TransactionsRecurringGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_refresh

> crate::models::TransactionsRefreshResponse transactions_refresh(transactions_refresh_request)
Refresh transaction data

`/transactions/refresh` is an optional endpoint for users of the Transactions product. It initiates an on-demand extraction to fetch the newest transactions for an Item. This on-demand extraction takes place in addition to the periodic extractions that automatically occur multiple times a day for any Transactions-enabled Item. If changes to transactions are discovered after calling `/transactions/refresh`, Plaid will fire a webhook: [`TRANSACTIONS_REMOVED`](https://plaid.com/docs/api/products/transactions/#transactions_removed) will be fired if any removed transactions are detected, and [`DEFAULT_UPDATE`](https://plaid.com/docs/api/products/transactions/#default_update) will be fired if any new transactions are detected. New transactions can be fetched by calling `/transactions/get`.  Access to `/transactions/refresh` in Production is specific to certain pricing plans. If you cannot access `/transactions/refresh` in Production, [contact Sales](https://www.plaid.com/contact) for assistance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transactions_refresh_request** | [**TransactionsRefreshRequest**](TransactionsRefreshRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionsRefreshResponse**](TransactionsRefreshResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_rules_create

> crate::models::TransactionsRulesCreateResponse transactions_rules_create(transactions_rules_create_request)
Create transaction category rule

The `/transactions/rules/create` endpoint creates transaction categorization rules.  Rules will be applied on the Item's transactions returned in `/transactions/get` response.  The product is currently in beta. To request access, contact transactions-feedback@plaid.com.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transactions_rules_create_request** | [**TransactionsRulesCreateRequest**](TransactionsRulesCreateRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionsRulesCreateResponse**](TransactionsRulesCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_rules_list

> crate::models::TransactionsRulesListResponse transactions_rules_list(transactions_rules_list_request)
Return a list of rules created for the Item associated with the access token.

The `/transactions/rules/list` returns a list of transaction rules created for the Item associated with the access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transactions_rules_list_request** | [**TransactionsRulesListRequest**](TransactionsRulesListRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionsRulesListResponse**](TransactionsRulesListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_rules_remove

> crate::models::TransactionsRulesRemoveResponse transactions_rules_remove(transactions_rules_remove_request)
Remove transaction rule

The `/transactions/rules/remove` endpoint is used to remove a transaction rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transactions_rules_remove_request** | [**TransactionsRulesRemoveRequest**](TransactionsRulesRemoveRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionsRulesRemoveResponse**](TransactionsRulesRemoveResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_sync

> crate::models::TransactionsSyncResponse transactions_sync(transactions_sync_request)
Get incremental transaction updates on an Item

The `/transactions/sync` endpoint returns item transactions as a set of delta updates. Subsequent calls to the endpoint using the cursor returned in the response will return new added, modified, and removed transactions since the last call to the endpoint  The product is currently in beta. To request access, contact transactions-feedback@plaid.com.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transactions_sync_request** | [**TransactionsSyncRequest**](TransactionsSyncRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionsSyncResponse**](TransactionsSyncResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_authorization_create

> crate::models::TransferAuthorizationCreateResponse transfer_authorization_create(transfer_authorization_create_request)
Create a transfer authorization

Use the `/transfer/authorization/create` endpoint to determine transfer failure risk.  In Plaid's sandbox environment the decisions will be returned as follows:    - To approve a transfer with null rationale code, make an authorization request with an `amount` less than the available balance in the account.    - To approve a transfer with the rationale code `MANUALLY_VERIFIED_ITEM`, create an Item in Link through the [Same Day Micro-deposits flow](https://plaid.com/docs/auth/coverage/testing/#testing-same-day-micro-deposits).      - To approve a transfer with the rationale code `LOGIN_REQUIRED`, [reset the login for an Item](https://plaid.com/docs/sandbox/#item_login_required).      - To decline a transfer with the rationale code `NSF`, the available balance on the account must be less than the authorization `amount`. See [Create Sandbox test data](https://plaid.com/docs/sandbox/user-custom/) for details on how to customize data in Sandbox.    - To decline a transfer with the rationale code `RISK`, the available balance on the account must be exactly $0. See [Create Sandbox test data](https://plaid.com/docs/sandbox/user-custom/) for details on how to customize data in Sandbox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_authorization_create_request** | [**TransferAuthorizationCreateRequest**](TransferAuthorizationCreateRequest.md) |  | [required] |

### Return type

[**crate::models::TransferAuthorizationCreateResponse**](TransferAuthorizationCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_cancel

> crate::models::TransferCancelResponse transfer_cancel(transfer_cancel_request)
Cancel a transfer

Use the `/transfer/cancel` endpoint to cancel a transfer.  A transfer is eligible for cancelation if the `cancellable` property returned by `/transfer/get` is `true`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_cancel_request** | [**TransferCancelRequest**](TransferCancelRequest.md) |  | [required] |

### Return type

[**crate::models::TransferCancelResponse**](TransferCancelResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_create

> crate::models::TransferCreateResponse transfer_create(transfer_create_request)
Create a transfer

Use the `/transfer/create` endpoint to initiate a new transfer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_create_request** | [**TransferCreateRequest**](TransferCreateRequest.md) |  | [required] |

### Return type

[**crate::models::TransferCreateResponse**](TransferCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_event_list

> crate::models::TransferEventListResponse transfer_event_list(transfer_event_list_request)
List transfer events

Use the `/transfer/event/list` endpoint to get a list of transfer events based on specified filter criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_event_list_request** | [**TransferEventListRequest**](TransferEventListRequest.md) |  | [required] |

### Return type

[**crate::models::TransferEventListResponse**](TransferEventListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_event_sync

> crate::models::TransferEventSyncResponse transfer_event_sync(transfer_event_sync_request)
Sync transfer events

`/transfer/event/sync` allows you to request up to the next 25 transfer events that happened after a specific `event_id`. Use the `/transfer/event/sync` endpoint to guarantee you have seen all transfer events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_event_sync_request** | [**TransferEventSyncRequest**](TransferEventSyncRequest.md) |  | [required] |

### Return type

[**crate::models::TransferEventSyncResponse**](TransferEventSyncResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_get

> crate::models::TransferGetResponse transfer_get(transfer_get_request)
Retrieve a transfer

The `/transfer/get` fetches information about the transfer corresponding to the given `transfer_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_get_request** | [**TransferGetRequest**](TransferGetRequest.md) |  | [required] |

### Return type

[**crate::models::TransferGetResponse**](TransferGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_intent_create

> crate::models::TransferIntentCreateResponse transfer_intent_create(transfer_intent_create_request)
Create a transfer intent object to invoke the Transfer UI

Use the `/transfer/intent/create` endpoint to generate a transfer intent object and invoke the Transfer UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_intent_create_request** | [**TransferIntentCreateRequest**](TransferIntentCreateRequest.md) |  | [required] |

### Return type

[**crate::models::TransferIntentCreateResponse**](TransferIntentCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_intent_get

> crate::models::TransferIntentGetResponse transfer_intent_get(request_body)
Retrieve more information about a transfer intent

Use the `/transfer/intent/get` endpoint to retrieve more information about a transfer intent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [required] |

### Return type

[**crate::models::TransferIntentGetResponse**](TransferIntentGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_list

> crate::models::TransferListResponse transfer_list(transfer_list_request)
List transfers

Use the `/transfer/list` endpoint to see a list of all your transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired transfers. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_list_request** | [**TransferListRequest**](TransferListRequest.md) |  | [required] |

### Return type

[**crate::models::TransferListResponse**](TransferListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_repayment_list

> crate::models::TransferRepaymentListResponse transfer_repayment_list(transfer_repayment_list_request)
Lists historical repayments

The `/transfer/repayment/list` endpoint fetches repayments matching the given filters. Repayments are returned in reverse-chronological order (most recent first) starting at the given `start_time`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_repayment_list_request** | [**TransferRepaymentListRequest**](TransferRepaymentListRequest.md) |  | [required] |

### Return type

[**crate::models::TransferRepaymentListResponse**](TransferRepaymentListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_repayment_return_list

> crate::models::TransferRepaymentReturnListResponse transfer_repayment_return_list(transfer_repayment_return_list_request)
List the returns included in a repayment

The `/transfer/repayment/return/list` endpoint retrieves the set of returns that were batched together into the specified repayment. The sum of amounts of returns retrieved by this request equals the amount of the repayment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_repayment_return_list_request** | [**TransferRepaymentReturnListRequest**](TransferRepaymentReturnListRequest.md) |  | [required] |

### Return type

[**crate::models::TransferRepaymentReturnListResponse**](TransferRepaymentReturnListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_sweep_get

> crate::models::TransferSweepGetResponse transfer_sweep_get(transfer_sweep_get_request)
Retrieve a sweep

The `/transfer/sweep/get` endpoint fetches a sweep corresponding to the given `sweep_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_sweep_get_request** | [**TransferSweepGetRequest**](TransferSweepGetRequest.md) |  | [required] |

### Return type

[**crate::models::TransferSweepGetResponse**](TransferSweepGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_sweep_list

> crate::models::TransferSweepListResponse transfer_sweep_list(transfer_sweep_list_request)
List sweeps

The `/transfer/sweep/list` endpoint fetches sweeps matching the given filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_sweep_list_request** | [**TransferSweepListRequest**](TransferSweepListRequest.md) |  | [required] |

### Return type

[**crate::models::TransferSweepListResponse**](TransferSweepListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_create

> crate::models::UserCreateResponse user_create(user_create_request)
Create user

Note: As of February 2022, the `/user/create` endpoint is in beta. Unless you have been opted-in, you will not have access to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_create_request** | [**UserCreateRequest**](UserCreateRequest.md) |  | [required] |

### Return type

[**crate::models::UserCreateResponse**](UserCreateResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_get

> crate::models::WalletGetResponse wallet_get(wallet_get_request)
Fetch an e-wallet

Fetch an e-wallet. The response includes the current balance. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_get_request** | [**WalletGetRequest**](WalletGetRequest.md) |  | [required] |

### Return type

[**crate::models::WalletGetResponse**](WalletGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_transaction_execute

> crate::models::WalletTransactionExecuteResponse wallet_transaction_execute(wallet_transaction_execute_request)
Execute a transaction using an e-wallet

Execute a transaction using the specified e-wallet. Specify the e-wallet to debit from, the counterparty to credit to, the idempotency key to prevent duplicate payouts, the amount and reference for the payout. The payouts are executed over the Faster Payment rails, where settlement usually only takes a few seconds. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_transaction_execute_request** | [**WalletTransactionExecuteRequest**](WalletTransactionExecuteRequest.md) |  | [required] |

### Return type

[**crate::models::WalletTransactionExecuteResponse**](WalletTransactionExecuteResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_transactions_list

> crate::models::WalletTransactionsListResponse wallet_transactions_list(wallet_transactions_list_request)
List e-wallet transactions

This endpoint lists the latest transactions of the specified e-wallet. Transactions are returned in descending order by the `created_at` time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_transactions_list_request** | [**WalletTransactionsListRequest**](WalletTransactionsListRequest.md) |  | [required] |

### Return type

[**crate::models::WalletTransactionsListResponse**](WalletTransactionsListResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_verification_key_get

> crate::models::WebhookVerificationKeyGetResponse webhook_verification_key_get(webhook_verification_key_get_request)
Get webhook verification key

Plaid signs all outgoing webhooks and provides JSON Web Tokens (JWTs) so that you can verify the authenticity of any incoming webhooks to your application. A message signature is included in the `Plaid-Verification` header.  The `/webhook_verification_key/get` endpoint provides a JSON Web Key (JWK) that can be used to verify a JWT.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_verification_key_get_request** | [**WebhookVerificationKeyGetRequest**](WebhookVerificationKeyGetRequest.md) |  | [required] |

### Return type

[**crate::models::WebhookVerificationKeyGetResponse**](WebhookVerificationKeyGetResponse.md)

### Authorization

[clientId](../README.md#clientId), [plaidVersion](../README.md#plaidVersion), [secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

