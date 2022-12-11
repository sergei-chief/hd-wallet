use std::collections::BTreeSet;
use std::io;

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
    let bitcoin_rpc = rpc::blockstream::BlockstreamRpc::with_default_url(transport);
    let bitcoin_address = hd_wallet.derive_default_address(CoinType::TWCoinTypeBitcoin);
    match bitcoin_rpc.transaction_count(&bitcoin_address).await {
        Ok(tx_count) => println!("{tx_count} transactions on {bitcoin_address} (BTC)"),
        Err(e) => eprint!("Error on getting Bitcoin address info: {e}"),
    }
}
