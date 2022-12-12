### TODO

 - Optimize transaction history request for Cosmos address.
Currently, it's designed suboptimal, and the transactions count is different from
what [mintscan](https://www.mintscan.io/) shows.

### How to run

1. Build [trustwallet/wallet-core](https://github.com/trustwallet/wallet-core) following
   the [guide](https://developer.trustwallet.com/wallet-core/developing-the-library/building).
2. Set the `TW_CORE_DIR` environment variable that points
   to the [trustwallet/wallet-core](https://github.com/trustwallet/wallet-core) directory.

```shell
export TW_CORE_DIR="/path/to/trustwallet/wallet-core"
```

3. Set the `ETHERSCAN_APIKEY` environment variable. For more info
   visit [Getting an API key](https://docs.etherscan.io/getting-started/viewing-api-usage-statistics)

```shell
export ETHERSCAN_APIKEY="API_KEY_HERE"
```

3. Run

```shell
cargo run -- mnemonic phrase or an entropy here
```
