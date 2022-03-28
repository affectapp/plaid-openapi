/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationStatusWebhook : Fired when the status of an income verification instance has changed. It will typically take several minutes for this webhook to fire after the end user has uploaded their documents in the Document Income flow.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncomeVerificationStatusWebhook {
    /// `\"INCOME\"`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `income_verification`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `income_verification_id` of the verification instance whose status is being reported.
    #[serde(rename = "income_verification_id")]
    pub income_verification_id: String,
    /// The Item ID associated with the verification.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// `VERIFICATION_STATUS_PROCESSING_COMPLETE`: The income verification status processing has completed. If the user uploaded multiple documents, this webhook will fire when all documents have finished processing. Call the `/income/verification/paystubs/get` endpoint and check the document metadata to see which documents were successfully parsed.  `VERIFICATION_STATUS_PROCESSING_FAILED`: A failure occurred when attempting to process the verification documentation.  `VERIFICATION_STATUS_PENDING_APPROVAL`: The income verification has been sent to the user for review.
    #[serde(rename = "verification_status")]
    pub verification_status: String,
}

impl IncomeVerificationStatusWebhook {
    /// Fired when the status of an income verification instance has changed. It will typically take several minutes for this webhook to fire after the end user has uploaded their documents in the Document Income flow.
    pub fn new(
        webhook_type: String,
        webhook_code: String,
        income_verification_id: String,
        item_id: String,
        verification_status: String,
    ) -> IncomeVerificationStatusWebhook {
        IncomeVerificationStatusWebhook {
            webhook_type,
            webhook_code,
            income_verification_id,
            item_id,
            verification_status,
        }
    }
}
