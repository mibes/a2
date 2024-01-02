//! Error and result module

use crate::response::Response;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// User request or Apple response JSON data was faulty.
    #[error("Error serializing to JSON: {0}")]
    SerializeError(#[from] serde_json::Error),

    /// A problem connecting to APNs servers.
    #[error("Error connecting to APNs: {0}")]
    ConnectionError(#[from] reqwest::Error),

    /// Couldn't generate an APNs token with the given key.
    #[error("Error creating a signature: {0}")]
    SignerError(#[from] openssl::error::ErrorStack),

    /// APNs couldn't accept the notification. Contains
    /// [Response](response/struct.Response.html) with additional
    /// information.
    #[error(
        "Notification was not accepted by APNs (reason: {})",
        .0.error
            .as_ref()
            .map(|e| e.reason.to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    )]
    ResponseError(Response),

    /// Invalid option values given in
    /// [NotificationOptions](request/notification/struct.NotificationOptions.html)
    #[error("Invalid options for APNs payload: {0}")]
    InvalidOptions(String),

    /// Error reading the certificate or private key.
    #[error("Error in reading a certificate file: {0}")]
    ReadError(#[from] io::Error),
}
