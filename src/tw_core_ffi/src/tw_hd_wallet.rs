use crate::tw_coin_type::{TWCoinType, TWCoinTypeRaw};
use crate::tw_data::{TWData, TWDataRaw};
use crate::tw_string::{TWString, TWStringRaw};
use std::fmt;

#[derive(Debug)]
pub struct InvalidMnemonic;

impl fmt::Display for InvalidMnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "Mnemonic is invalid") }
}

#[derive(Debug)]
pub struct InvalidEntropy;

impl fmt::Display for InvalidEntropy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "Entropy is invalid") }
}

#[derive(Debug)]
pub struct TWHDWallet {
    raw: *mut TWHDWalletRaw,
}

impl TWHDWallet {
    pub fn with_mnemonic<M, P>(mnemonic: M, passphrase: P) -> Result<TWHDWallet, InvalidMnemonic>
    where
        M: Into<TWString>,
        P: Into<TWString>,
    {
        const CHECK_MNEMONIC: bool = true;

        let mnemonic = mnemonic.into();
        let passphrase = passphrase.into();

        let raw = unsafe {
            TWHDWalletCreateWithMnemonicCheck(
                mnemonic.as_ptr(),
                passphrase.as_ptr(),
                CHECK_MNEMONIC,
            )
        };
        if raw.is_null() {
            return Err(InvalidMnemonic);
        }

        Ok(TWHDWallet { raw })
    }

    pub fn with_entropy<E, P>(entropy: E, passphrase: P) -> Result<TWHDWallet, InvalidEntropy>
    where
        E: Into<TWData>,
        P: Into<TWString>,
    {
        let entropy = entropy.into();
        let passphrase = passphrase.into();

        let raw = unsafe { TWHDWalletCreateWithEntropy(entropy.as_ptr(), passphrase.as_ptr()) };
        if raw.is_null() {
            return Err(InvalidEntropy);
        }

        Ok(TWHDWallet { raw })
    }

    pub fn derive_default_address(&self, coin: TWCoinType) -> String {
        let tw_string_raw = unsafe { TWHDWalletGetAddressForCoin(self.raw, coin.as_raw()) };
        TWString::from_raw(tw_string_raw)
            .to_string()
            .expect("'TWHDWalletGetAddressForCoin' should have returned a non UTF-8 string")
    }
}

impl Drop for TWHDWallet {
    fn drop(&mut self) { unsafe { TWHDWalletDelete(self.raw) } }
}

#[repr(C)]
pub(crate) struct TWHDWalletRaw {
    private: [u8; 0],
}

extern "C" {
    /// Returns `nullptr` if the given `mnemonic` is invalid.
    fn TWHDWalletCreateWithMnemonicCheck(
        mnemonic: *const TWStringRaw,
        passphrase: *const TWStringRaw,
        check: bool,
    ) -> *mut TWHDWalletRaw;

    /// Returns `nullptr` if the given `mnemonic` is invalid.
    fn TWHDWalletCreateWithEntropy(
        entropy: *const TWDataRaw,
        passphrase: *const TWStringRaw,
    ) -> *mut TWHDWalletRaw;

    /// # Safe
    ///
    /// `TWHDWalletGetAddressForCoin` panics if the given `coin` is unknown.
    fn TWHDWalletGetAddressForCoin(
        wallet: *const TWHDWalletRaw,
        coin: TWCoinTypeRaw,
    ) -> *mut TWStringRaw;

    fn TWHDWalletDelete(wallet: *mut TWHDWalletRaw);
}

#[cfg(test)]
mod tests {
    use super::*;

    const PASSPHRASE: &str = "";

    #[test]
    fn test_hd_wallet_with_mnemonic_default_pass() {
        let valid_mnemonics = [
            "oil oil oil oil oil oil oil oil oil oil oil oil",
            "pencil destroy loan write history tattoo record consider resemble assume rude life", // 12 words
            "long dumb grain gesture that design type diary crucial carry comic smile poet van core", // 15 words
            "trick plunge bless pen tone elder velvet squirrel pluck vital man coin essence charge plunge mutual between return", // 18 words
            "rhythm bitter siren often olympic update zoo memory mother decrease acoustic midnight symbol two execute pony cover room tiny plate pupil", // 21 words
            "leopard melt search path this pluck rapid hope clever sphere fiction pact affair maid chronic donor priority pride reform above force assault return dirt", // 24 words
        ];

        for mnemonic in valid_mnemonics {
            TWHDWallet::with_mnemonic(mnemonic, PASSPHRASE)
                .expect(&format!("'{mnemonic}' is expected to be a valid mnemonic"));
        }

        let invalid_mnemonics = [
            "oil oil oil oil oil oil oil oil oil oil oil oil text",
            "99d33a674ce99d33a674ce99d33a674c",
        ];

        for mnemonic in invalid_mnemonics {
            TWHDWallet::with_mnemonic(mnemonic, PASSPHRASE).expect_err(&format!(
                "'{mnemonic}' is expected to be an invalid mnemonic"
            ));
        }
    }

    #[test]
    fn test_hd_wallet_with_entropy_default_pass() {
        let valid_entropies = [
            "99d33a674ce99d33a674ce99d33a674c",
            "99d33a674ce99d33a674ce99d33a674c99d33a674ce99d33a674ce99d33a674c",
        ];

        for entropy in valid_entropies {
            let entropy = hex::decode(entropy).unwrap();

            TWHDWallet::with_entropy(entropy.as_slice(), PASSPHRASE)
                .expect(&format!("'{entropy:?}' is expected to be a valid entropy"));
        }

        let invalid_entropies = ["99d3", "99d33a674ce99d33a674ce99d33a67"];

        for entropy in invalid_entropies {
            let entropy = hex::decode(entropy).unwrap();

            TWHDWallet::with_entropy(entropy.as_slice(), PASSPHRASE).expect_err(&format!(
                "'{entropy:?}' is expected to be an invalid entropy"
            ));
        }
    }

    #[test]
    fn test_hd_wallet_derive_address() {
        let tw_hd_wallet = TWHDWallet::with_mnemonic(
            "oil oil oil oil oil oil oil oil oil oil oil oil",
            PASSPHRASE,
        )
        .unwrap();
        let actual = tw_hd_wallet.derive_default_address(TWCoinType::TWCoinTypeBitcoin);
        assert_eq!(actual, "bc1q98wufxmtfh5qlk7fe5dzy2z8cflvqjysrh4fx2");
    }
}
