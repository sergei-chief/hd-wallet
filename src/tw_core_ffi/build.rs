//! Inspired by https://github.com/trustwallet/wallet-core/blob/master/samples/rust/src/build.rs

use std::env;
use std::path::Path;

/// The `trustwallet/wallet-core` build guide.
const WALLET_CORE_BUILDING: &str =
    "https://developer.trustwallet.com/wallet-core/developing-the-library/building";

/// libs to link with, in reverse dependency order.
const LIBS: [&str; 4] = [
    "TrustWalletCore",
    "TrezorCrypto",
    "protobuf",
    "wallet_core_rs",
];

fn main() {
    let tw_core_dir = env::var("TW_CORE_DIR").expect("'TW_CORE_DIR' is not set");

    // Check if the `wallet-core/build` directory exists.
    let tw_core_build = Path::new(&tw_core_dir).join("build");
    if !tw_core_build.is_dir() {
        panic!("{tw_core_build:?} doesn't exist. For more info visit {WALLET_CORE_BUILDING}");
    }

    println!("cargo:rustc-link-search=native={tw_core_dir}/build",);
    println!("cargo:rustc-link-search=native={tw_core_dir}/build/trezor-crypto",);
    println!("cargo:rustc-link-search=native={tw_core_dir}/build/local/lib",);

    // Libraries; order matters
    for i in 0..LIBS.len() {
        println!("cargo:rustc-link-lib={}", LIBS[i]);
    }
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        // "linux", etc
        println!("cargo:rustc-link-lib=stdc++");
    }
}
