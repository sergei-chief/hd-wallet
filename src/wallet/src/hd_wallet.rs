use std::ops::RangeInclusive;
use tw_core_ffi::tw_hd_wallet::{InvalidEntropy, InvalidMnemonic, TWHDWallet};

pub struct HDWallet {
    inner: TWHDWallet,
}

impl HDWallet {
    /// Creates `HDWallet` with the given `mnemonic` and `passphrase`.
    pub fn with_mnemonic(
        mnemonic: String,
        passphrase: String,
    ) -> Result<HDWallet, InvalidMnemonic> {
        let inner = TWHDWallet::with_mnemonic(mnemonic, passphrase)?;
        Ok(HDWallet { inner })
    }

    /// Creates `HDWallet` with the given mnemonic `entropy` (hex string, 16-32 bytes) and `passphrase`.
    pub fn with_entropy(entropy: &str, passphrase: String) -> Result<HDWallet, InvalidEntropy> {
        const VALID_ENTROPY_LEN: RangeInclusive<usize> = 16..=32;

        // Strip the `0x` prefix if it presents.
        let entropy_without_prefix = entropy.strip_prefix("0x").unwrap_or(entropy);

        let entropy_data = hex::decode(entropy_without_prefix).map_err(|_| InvalidEntropy)?;

        if !VALID_ENTROPY_LEN.contains(&entropy_data.len()) {
            return Err(InvalidEntropy);
        }

        let inner = TWHDWallet::with_entropy(entropy_data, passphrase)?;
        Ok(HDWallet { inner })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PASSPHRASE: &str = "";

    #[test]
    fn test_hd_wallet_with_entropy() {
        let valid_entropies = [
            "99d33a674ce99d33a674ce99d33a674c",
            "0x99d33a674ce99d33a674ce99d33a674c",
            "99d33a674ce99d33a674ce99d33a674c99d33a674ce99d33a674ce99d33a674c",
            "0x99d33a674ce99d33a674ce99d33a674c99d33a674ce99d33a674ce99d33a674c",
        ];

        for entropy in valid_entropies {
            HDWallet::with_entropy(entropy, PASSPHRASE.to_string())
                .expect(&format!("{entropy} is expected to be a valid entropy"));
        }
    }
}
