use crate::http::{HttpError, HttpTransport};
use http::uri::InvalidUri;
use http::Response;
use hyper::Uri;
use serde::Deserialize;

pub const BLOCKSTREAM_URL: &str = "https://blockstream.info";

/// https://github.com/bitcoin/bitcoin/blob/master/src/consensus/amount.h#L11-L12
#[allow(dead_code)]
pub type Satoshis = i64;

pub struct BlockstreamRpc<T> {
    transport: T,
    url: String,
}

impl<T> BlockstreamRpc<T>
where
    T: HttpTransport + Sync,
{
    pub fn with_default_url(transport: T) -> Self {
        BlockstreamRpc::with_url(transport, BLOCKSTREAM_URL.to_string())
            .expect("'BLOCKSTREAM_URL' is expected to be a valid URL")
    }

    pub fn with_url(transport: T, url: String) -> Result<Self, InvalidUri> {
        // Check if the given `url` is correct.
        url.parse::<Uri>()?;
        Ok(BlockstreamRpc { transport, url })
    }

    /// Requests the count of the address transactions.
    pub async fn transaction_count(&self, address: &str) -> Result<usize, HttpError> {
        self.address_info(address)
            .await
            .map(|addr_info| addr_info.chain_stats.tx_count)
    }

    /// Requests the the address info.
    pub async fn address_info(&self, address: &str) -> Result<AddressInfo, HttpError> {
        let uri = format!("{}/api/address/{address}", self.url).parse()?;
        let res: Response<AddressInfo> = self.transport.get_json(uri).await?;

        // TODO check if the response is OK(200).
        let (_parts, address_info) = res.into_parts();
        Ok(address_info)
    }
}

#[derive(Deserialize)]
pub struct AddressInfo {
    #[allow(dead_code)]
    address: String,
    chain_stats: ChainStats,
}

/// Currently, we're interested in the address's balance and TX count only.
#[derive(Deserialize)]
pub struct ChainStats {
    #[allow(dead_code)]
    funded_txo_sum: Satoshis,
    tx_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blockstream_rpc() {
        let transport = crate::http::HttpBuilder::build();
        let rpc = BlockstreamRpc::with_default_url(transport);

        let res = rpc
            .address_info("bc1qpjult34k9spjfym8hss2jrwjgf0xjf40ze0pp8")
            .await
            .unwrap();
        assert_eq!(res.address, "bc1qpjult34k9spjfym8hss2jrwjgf0xjf40ze0pp8");
        assert!(res.chain_stats.tx_count > 0);
    }
}
