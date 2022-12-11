use async_trait::async_trait;
use hyper::client::{Client, HttpConnector};
use serde::de::DeserializeOwned;
use std::fmt;

pub use http::{uri::InvalidUri, Response, Uri};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

#[derive(Debug)]
pub enum HttpError {
    InvalidUri(String),
    InvalidPayload(String),
    TransportError(String),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::InvalidUri(e) => write!(f, "Invalid URI: {e}"),
            HttpError::InvalidPayload(e) => write!(f, "Invalid response payload: {e}"),
            HttpError::TransportError(e) => write!(f, "Transport error: {e}"),
        }
    }
}

impl From<hyper::Error> for HttpError {
    fn from(e: hyper::Error) -> Self { HttpError::TransportError(e.to_string()) }
}

impl From<InvalidUri> for HttpError {
    fn from(e: InvalidUri) -> Self { HttpError::InvalidUri(e.to_string()) }
}

/// An `HttpTransport` builder.
/// This pattern allows us to change the transport for the whole app in one line.
pub struct HttpBuilder;

impl HttpBuilder {
    pub fn build() -> impl HttpTransport { HyperTransport::default() }
}

#[async_trait]
pub trait HttpTransport {
    /// Sends an `HTTP GET` request.
    async fn get(&self, uri: Uri) -> Result<Response<Vec<u8>>, HttpError>;

    /// Sends an `HTTP GET` request, parses payload as a Json.
    async fn get_json<T>(&self, uri: Uri) -> Result<Response<T>, HttpError>
    where
        T: DeserializeOwned,
    {
        let (parts, body) = self.get(uri).await?.into_parts();
        let json =
            serde_json::from_slice(&body).map_err(|e| HttpError::InvalidPayload(e.to_string()))?;
        Ok(Response::from_parts(parts, json))
    }
}

pub struct HyperTransport {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Default for HyperTransport {
    fn default() -> Self {
        let connector = HttpsConnectorBuilder::default()
            .with_native_roots()
            .https_or_http()
            .enable_http2()
            .build();
        HyperTransport {
            client: Client::builder().build(connector),
        }
    }
}

#[async_trait]
impl HttpTransport for HyperTransport {
    async fn get(&self, uri: Uri) -> Result<Response<Vec<u8>>, HttpError> {
        let (parts, body) = self.client.get(uri).await?.into_parts();
        let bytes = hyper::body::to_bytes(body).await?;
        Ok(Response::from_parts(parts, bytes.into()))
    }
}
