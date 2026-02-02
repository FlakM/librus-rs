//! Error types for the Librus API client.

use thiserror::Error;

/// Errors that can occur when using the Librus API client.
///
/// # Example
///
/// ```rust,no_run
/// use librus_rs::{Client, Error};
///
/// # async fn example() {
/// match Client::from_env().await {
///     Ok(_) => println!("Success"),
///     Err(Error::MissingEnvVar(var)) => eprintln!("Missing {}", var),
///     Err(Error::Authentication) => eprintln!("Bad credentials"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// # }
/// ```
#[derive(Debug, Error)]
pub enum Error {
    /// Authentication with Librus failed due to invalid credentials or server error.
    #[error("authentication failed: invalid credentials or server error")]
    Authentication,

    /// Required environment variable is not set.
    ///
    /// Returned by [`Client::from_env()`](crate::Client::from_env) when
    /// `LIBRUS_USERNAME` or `LIBRUS_PASSWORD` is missing.
    #[error("environment variable `{0}` is not set")]
    MissingEnvVar(&'static str),

    /// Required credential is missing from the builder.
    ///
    /// Returned by [`ClientBuilder::build()`](crate::ClientBuilder::build) when
    /// username or password was not provided.
    #[error("missing required credential: {0}")]
    MissingCredentials(&'static str),

    /// HTTP client construction failed.
    #[error("failed to build HTTP client: {0}")]
    HttpClient(#[source] reqwest::Error),

    /// HTTP request failed due to network or connection error.
    #[error("request failed: {0}")]
    Request(#[source] reqwest::Error),

    /// API returned an error response.
    ///
    /// Contains the HTTP status code and response body for debugging.
    #[error("API error (status {status}): {body}")]
    ApiError {
        /// HTTP status code returned by the API.
        status: u16,
        /// Response body content.
        body: String,
    },

    /// Failed to parse API response as JSON.
    ///
    /// This usually indicates an unexpected response format from the API.
    #[error("failed to parse response: {source}")]
    Parse {
        /// The underlying JSON parsing error.
        #[source]
        source: serde_json::Error,
        /// The raw response body that failed to parse.
        body: String,
    },
}
