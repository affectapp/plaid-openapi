/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// WalletTransactionStatus : The status of the transaction.  `INITIATED`: This is the initial state of all transactions. It indicates that the transaction has been initiated and is currently being processed.  `EXECUTED`: The transaction has been successfully executed.  `FAILED`: The transaction failed to process successfully. This is a terminal status.  `BLOCKED`: The transaction has been blocked for violating compliance rules. This is a terminal status.

/// The status of the transaction.  `INITIATED`: This is the initial state of all transactions. It indicates that the transaction has been initiated and is currently being processed.  `EXECUTED`: The transaction has been successfully executed.  `FAILED`: The transaction failed to process successfully. This is a terminal status.  `BLOCKED`: The transaction has been blocked for violating compliance rules. This is a terminal status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WalletTransactionStatus {
    #[serde(rename = "INITIATED")]
    INITIATED,
    #[serde(rename = "EXECUTED")]
    EXECUTED,
    #[serde(rename = "BLOCKED")]
    BLOCKED,
    #[serde(rename = "FAILED")]
    FAILED,
}

impl ToString for WalletTransactionStatus {
    fn to_string(&self) -> String {
        match self {
            Self::INITIATED => String::from("INITIATED"),
            Self::EXECUTED => String::from("EXECUTED"),
            Self::BLOCKED => String::from("BLOCKED"),
            Self::FAILED => String::from("FAILED"),
        }
    }
}

impl Default for WalletTransactionStatus {
    fn default() -> WalletTransactionStatus {
        Self::INITIATED
    }
}
