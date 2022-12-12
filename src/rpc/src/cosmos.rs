use crate::http::{HttpError, HttpTransport};
use http::uri::InvalidUri;
use http::{Response, Uri};
use serde::Deserialize;
use std::collections::HashSet;

const COSMOS_URL: &str = "https://api.cosmos.network";

pub struct CosmosRpc<'a, T> {
    transport: &'a T,
    url: String,
}

impl<'a, T> CosmosRpc<'a, T>
where
    T: HttpTransport + Sync,
{
    pub fn with_default_url(transport: &'a T) -> Self {
        CosmosRpc::with_url(transport, COSMOS_URL.to_string())
            .expect("'ETHERSCAN_URL' is expected to be a valid URL")
    }

    pub fn with_url(transport: &'a T, url: String) -> Result<Self, InvalidUri> {
        // Check if the given `url` is correct.
        url.parse::<Uri>()?;
        Ok(CosmosRpc { transport, url })
    }

    /// Requests the count of the address transactions.
    /// TODO this is suboptimal to fetch all transactions, but I couldn't find another quick solution.
    pub async fn transaction_count(&self, address: &str) -> Result<usize, HttpError> {
        let spender_txs = self.request_txs(&EventFilter::spender(address)).await?;
        let receiver_txs = self.request_txs(&EventFilter::receiver(address)).await?;

        // Determine unique transaction hashes to get a total number of address transactions.
        let unique_txs: HashSet<_> = spender_txs
            .into_iter()
            .chain(receiver_txs.into_iter())
            .collect();

        Ok(unique_txs.len())
    }

    /// Requests the the address info.
    async fn request_txs(&self, events: &str) -> Result<Vec<TxInfo>, HttpError> {
        /// Set the limit to `0` to fetch all transactions.
        const LIMIT: usize = 0;

        let url = &self.url;
        let uri = format!("{url}/cosmos/tx/v1beta1/txs?events={events}&pagination.limit={LIMIT}")
            .parse()?;

        let res: Response<TxsResult> = self.transport.get_json(uri).await?;
        // TODO check if the response is OK(200) and if the `EtherscanResponse::message` is "OK".
        let (_parts, res) = res.into_parts();

        Ok(res.tx_responses)
    }
}

struct EventFilter;

impl EventFilter {
    fn spender(address: &str) -> String { format!("coin_spent.spender='{address}'") }

    fn receiver(address: &str) -> String { format!("coin_received.receiver='{address}'") }
}

/// We're currently interested in the `tx_responses` field, so ignore other ones.
#[derive(Deserialize)]
struct TxsResult {
    tx_responses: Vec<TxInfo>,
}

/// We're currently interested in the `txhash` field, so ignore other ones.
#[derive(Deserialize, Eq, Hash, PartialEq)]
struct TxInfo {
    txhash: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cosmos_transaction_count() {
        let transport = crate::http::HttpBuilder::build();
        let rpc = CosmosRpc::with_default_url(&transport);

        let actual = rpc
            .transaction_count("cosmos1mzfn3lk6f6vu6hnazc5fazxn9eme5acmay4p6m")
            .await
            .unwrap();
        assert!(actual >= 34, "actual={}", actual);

        let actual = rpc
            .transaction_count("cosmos14q3kddx7rus4t7c6gpfjxezqjj8ka6t4tkqhey")
            .await
            .unwrap();
        assert_eq!(actual, 0);
    }
}
