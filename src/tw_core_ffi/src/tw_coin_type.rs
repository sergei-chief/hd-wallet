use strum::{EnumIter, IntoEnumIterator};

pub(crate) type TWCoinTypeRaw = u32;

/// Coin type for Level 2 of BIP44.
///
/// https://github.com/satoshilabs/slips/blob/master/slip-0044.md
///
/// TODO add a test that iterates over the coin types
/// and calls `TWHDWalletGetAddressForCoin` to check if every coin type is known by the `trustwallet/wallet-core`.
#[repr(u32)]
#[derive(Clone, Copy, Debug, EnumIter)]
pub enum TWCoinType {
    TWCoinTypeAeternity = 457,
    TWCoinTypeAion = 425,
    TWCoinTypeBinance = 714,
    TWCoinTypeBitcoin = 0,
    TWCoinTypeBitcoinCash = 145,
    TWCoinTypeBitcoinGold = 156,
    TWCoinTypeCallisto = 820,
    TWCoinTypeCardano = 1815, // Note: Cardano Shelley testnet uses purpose 1852 (not 44) 1852/1815
    TWCoinTypeCosmos = 118,
    TWCoinTypeDash = 5,
    TWCoinTypeDecred = 42,
    TWCoinTypeDigiByte = 20,
    TWCoinTypeDogecoin = 3,
    TWCoinTypeEOS = 194,
    TWCoinTypeEthereum = 60,
    TWCoinTypeEthereumClassic = 61,
    TWCoinTypeFIO = 235,
    TWCoinTypeGoChain = 6060,
    TWCoinTypeGroestlcoin = 17,
    TWCoinTypeICON = 74,
    TWCoinTypeIoTeX = 304,
    TWCoinTypeKava = 459,
    TWCoinTypeKin = 2017,
    TWCoinTypeLitecoin = 2,
    TWCoinTypeMonacoin = 22,
    TWCoinTypeNebulas = 2718,
    TWCoinTypeNULS = 8964,
    TWCoinTypeNano = 165,
    TWCoinTypeNEAR = 397,
    TWCoinTypeNimiq = 242,
    TWCoinTypeOntology = 1024,
    TWCoinTypePOANetwork = 178,
    TWCoinTypeQtum = 2301,
    TWCoinTypeXRP = 144,
    TWCoinTypeSolana = 501,
    TWCoinTypeStellar = 148,
    TWCoinTypeTezos = 1729,
    TWCoinTypeTheta = 500,
    TWCoinTypeThunderToken = 1001,
    TWCoinTypeNEO = 888,
    TWCoinTypeTomoChain = 889,
    TWCoinTypeTron = 195,
    TWCoinTypeVeChain = 818,
    TWCoinTypeViacoin = 14,
    TWCoinTypeWanchain = 5718350,
    TWCoinTypeZcash = 133,
    TWCoinTypeFiro = 136,
    TWCoinTypeZilliqa = 313,
    TWCoinTypeZelcash = 19167,
    TWCoinTypeRavencoin = 175,
    TWCoinTypeWaves = 5741564,
    TWCoinTypeTerra = 330,        // see also TerraV2
    TWCoinTypeTerraV2 = 10000330, // see also Terra
    TWCoinTypeHarmony = 1023,
    TWCoinTypeAlgorand = 283,
    TWCoinTypeKusama = 434,
    TWCoinTypePolkadot = 354,
    TWCoinTypeFilecoin = 461,
    TWCoinTypeElrond = 508,
    TWCoinTypeBandChain = 494,
    TWCoinTypeSmartChainLegacy = 10000714,
    TWCoinTypeSmartChain = 20000714,
    TWCoinTypeOasis = 474,
    TWCoinTypePolygon = 966,
    TWCoinTypeTHORChain = 931,
    TWCoinTypeBluzelle = 483,
    TWCoinTypeOptimism = 10000070,
    TWCoinTypeZksync = 10000280,
    TWCoinTypeArbitrum = 10042221,
    TWCoinTypeECOChain = 10000553,
    TWCoinTypeAvalancheCChain = 10009000,
    TWCoinTypeXDai = 10000100,
    TWCoinTypeFantom = 10000250,
    TWCoinTypeCryptoOrg = 394,
    TWCoinTypeCelo = 52752,
    TWCoinTypeRonin = 10002020,
    TWCoinTypeOsmosis = 10000118,
    TWCoinTypeECash = 899,
    TWCoinTypeCronosChain = 10000025,
    TWCoinTypeSmartBitcoinCash = 10000145,
    TWCoinTypeKuCoinCommunityChain = 10000321,
    TWCoinTypeBoba = 10000288,
    TWCoinTypeMetis = 1001088,
    TWCoinTypeAurora = 1323161554,
    TWCoinTypeEvmos = 10009001,
    TWCoinTypeNativeEvmos = 20009001,
    TWCoinTypeMoonriver = 10001285,
    TWCoinTypeMoonbeam = 10001284,
    TWCoinTypeKavaEvm = 10002222,
    TWCoinTypeKlaytn = 10008217,
    TWCoinTypeMeter = 18000,
    TWCoinTypeOKXChain = 996,
    TWCoinTypeNervos = 309,
    TWCoinTypeEverscale = 396,
    TWCoinTypeAptos = 637,
    TWCoinTypeHedera = 3030,
    TWCoinTypeSecret = 529,
}

impl TWCoinType {
    pub fn as_raw(&self) -> TWCoinTypeRaw { *self as TWCoinTypeRaw }

    /// Returns an iterator of all supported coins.
    /// This static method allows to avoid importing the `IntoEnumIterator` trait.
    pub fn iter_coins() -> impl Iterator<Item = TWCoinType> { TWCoinType::iter() }
}
