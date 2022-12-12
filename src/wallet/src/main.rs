use std::collections::BTreeSet;
use std::{env, io};

mod hd_wallet;
mod wallet_input;

use hd_wallet::{CoinType, HDWallet};
use wallet_input::WalletInput;

const DEFAULT_PASSPHRASE: &str = "";

fn help() {
    println!("Input: As command-line argument, a BIP39 mnemonic or a mnemonic entropy (hex string) is also accepted (16-32 bytes)");
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let input = match WalletInput::parse_cmd_args() {
        Ok(input) => input,
        Err(e) => {
            help();
            return Err(e.into());
        }
    };

    let hd_wallet = match input {
        WalletInput::Mnemonic(mnemonic) => {
            HDWallet::with_mnemonic(mnemonic, DEFAULT_PASSPHRASE.to_string())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?
        }
        WalletInput::Entropy(entropy) => {
            HDWallet::with_entropy(&entropy, DEFAULT_PASSPHRASE.to_string())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?
        }
    };

    // Order the addresses alphabetically and remove duplicates by using `BTreeSet`.
    let addresses: BTreeSet<_> = hd_wallet
        .derive_default_addresses(CoinType::iter_coins())
        .collect();

    print_activity(&hd_wallet).await;

    println!("\nAll addresses:");
    for address in addresses {
        println!("  {address}");
    }

    Ok(())
}

/// Prints out, if, for the BTC, ETH, and ATOM addresses there is a current balance
/// and whether there were any transactions on the address.
async fn print_activity(hd_wallet: &HDWallet) {
    println!("Activity:");

    let transport = rpc::http::HttpBuilder::build();
    let bitcoin_address = hd_wallet.derive_default_address(CoinType::TWCoinTypeBitcoin);
    let eth_address = hd_wallet.derive_default_address(CoinType::TWCoinTypeEthereum);
    let cosmos_address = hd_wallet.derive_default_address(CoinType::TWCoinTypeCosmos);

    print_btc_activity(&bitcoin_address, &transport).await;

    match env::var("ETHERSCAN_APIKEY") {
        Ok(api_key) => print_eth_activity(&eth_address, &transport, api_key).await,
        Err(_) => eprintln!("Set 'ETHERSCAN_APIKEY' environment variable to show an ETH activity"),
    }

    print_cosmos_activity(&cosmos_address, &transport).await;
}

async fn print_btc_activity<T>(bitcoin_address: &str, transport: &T)
where
    T: rpc::http::HttpTransport + Sync,
{
    let bitcoin_rpc = rpc::blockstream::BlockstreamRpc::with_default_url(transport);
    match bitcoin_rpc.transaction_count(bitcoin_address).await {
        Ok(tx_count) => println!("  {tx_count} transactions on {bitcoin_address} (BTC)"),
        Err(e) => eprintln!("Error on getting Bitcoin address info: {e}"),
    }
}

async fn print_eth_activity<T>(eth_address: &str, transport: &T, api_key: String)
where
    T: rpc::http::HttpTransport + Sync,
{
    let eth_rpc = rpc::etherscan::EtherscanRpc::with_default_url(transport, api_key);
    match eth_rpc.transaction_count(eth_address).await {
        Ok(tx_count) => println!("  {tx_count} transactions on {eth_address} (ETH)"),
        Err(e) => eprintln!("Error on getting Ethereum address info: {e}"),
    }
}

async fn print_cosmos_activity<T>(cosmos_address: &str, transport: &T)
where
    T: rpc::http::HttpTransport + Sync,
{
    let cosmos_rpc = rpc::cosmos::CosmosRpc::with_default_url(transport);
    match cosmos_rpc.transaction_count(cosmos_address).await {
        Ok(tx_count) => println!("  {tx_count} transactions on {cosmos_address} (ADA)"),
        Err(e) => eprintln!("Error on getting Cosmos address info: {e}"),
    }
}
