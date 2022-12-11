use crate::http::{HttpError, HttpTransport};
use http::uri::InvalidUri;
use http::{Response, Uri};
use serde::Deserialize;

const ETHERSCAN_URL: &str = "https://api.etherscan.io";

pub struct EtherscanRpc<'a, T> {
    transport: &'a T,
    api_key: String,
    url: String,
}

impl<'a, T> EtherscanRpc<'a, T>
where
    T: HttpTransport + Sync,
{
    pub fn with_default_url(transport: &'a T, api_key: String) -> Self {
        EtherscanRpc::with_url(transport, api_key, ETHERSCAN_URL.to_string())
            .expect("'ETHERSCAN_URL' is expected to be a valid URL")
    }

    pub fn with_url(transport: &'a T, api_key: String, url: String) -> Result<Self, InvalidUri> {
        // Check if the given `url` is correct.
        url.parse::<Uri>()?;
        Ok(EtherscanRpc {
            transport,
            api_key,
            url,
        })
    }

    /// Requests the count of the address transactions.
    pub async fn transaction_count(&self, address: &str) -> Result<usize, HttpError> {
        self.txlist(address).await.map(|txs| txs.len())
    }

    /// Requests the the address info.
    pub async fn txlist(&self, address: &str) -> Result<Vec<TxListItem>, HttpError> {
        let action = "txlist";
        let uri = self.address_request_uri(action, address)?;
        let res: Response<EtherscanResponse<Vec<TxListItem>>> =
            self.transport.get_json(uri).await?;

        // TODO check if the response is OK(200) and if the `EtherscanResponse::message` is "OK".
        let (_parts, ether_res) = res.into_parts();
        Ok(ether_res.result)
    }

    fn address_request_uri(&self, action: &str, address: &str) -> Result<Uri, InvalidUri> {
        let url = &self.url;
        let apikey = &self.api_key;
        format!("{url}/api?module=account&action={action}&apikey={apikey}&address={address}")
            .parse()
    }
}

#[derive(Deserialize)]
struct EtherscanResponse<T> {
    #[allow(dead_code)]
    status: String,
    #[allow(dead_code)]
    message: String,
    result: T,
}

/// We're currently interested in the count of transactions, so leave the structure fields empty.
#[derive(Deserialize)]
pub struct TxListItem {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_etherscan_rpc() {
        let api_key = env::var("ETHERSCAN_APIKEY")
            .expect("'ETHERSCAN_APIKEY' environment variable is not set");

        let transport = crate::http::HttpBuilder::build();
        let rpc = EtherscanRpc::with_default_url(&transport, api_key);

        let actual = rpc
            .transaction_count("0x60c2A43Cc69658eC4b02a65A07623D7192166F4e")
            .await
            .unwrap();
        // `0x60c2A43Cc69658eC4b02a65A07623D7192166F4e` has 6 transactions at the moment when the test has been written.
        assert!(actual >= 6, "actual={}", actual);
    }
}
