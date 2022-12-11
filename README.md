### How to run

1. Build [trustwallet/wallet-core](https://github.com/trustwallet/wallet-core) following
   the [guide](https://developer.trustwallet.com/wallet-core/developing-the-library/building).
2. Set the `TW_CORE_DIR` environment variable that points
   to the [trustwallet/wallet-core](https://github.com/trustwallet/wallet-core) directory.

```shell
export TW_CORE_DIR="/path/to/trustwallet/wallet-core"
```

3. Run
```shell
cargo run -- mnemonic phrase or an entropy here
```
