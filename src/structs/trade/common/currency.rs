use bc_utils_lg::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrencyType {
    Fiat,
    Crypto,
    CommodityBacked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrencyCode {
    pub code: Ustr,
    pub iso4217: Option<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Currency {
    pub precision: u8,
    pub code: CurrencyCode,
    pub r#type: CurrencyType,
}

macro_rules! currency_constants {
    ($(
        $accessor:ident => $precision:literal, $code:literal, $iso4217:expr, $currency_type:ident;
    )+) => {
      impl Currency {
        $(
            #[allow(non_snake_case)]
            pub fn $accessor() -> Self {
                static LOCK: OnceLock<Currency> =  OnceLock::new();
                *LOCK.get_or_init(
                    || Self{
                        precision: $precision,
                        code: CurrencyCode {
                            code: Ustr::from($code),
                            iso4217: $iso4217,
                        },
                        r#type: CurrencyType::$currency_type,
                    }
                )
            }
        )+

    }

    pub static CURRENCY_MAP: LazyLock<Mutex<MAP<String, Currency,>>> = LazyLock::new(|| {
        let mut map = MAP::default();
        $(map.insert(Currency::$accessor().code.code.to_string(), Currency::$accessor());)+
        Mutex::new(map)
    });
    };
}

currency_constants! {
    // Fiat currencies
    AUD => 2, "AUD", Some(36), Fiat;
    BRL => 2, "BRL", Some(986), Fiat;
    CAD => 2, "CAD", Some(124), Fiat;
    CHF => 2, "CHF", Some(756), Fiat;
    CNY => 2, "CNY", Some(156), Fiat;
    CNH => 2, "CNH", Some(0), Fiat;
    CZK => 2, "CZK", Some(203), Fiat;
    DKK => 2, "DKK", Some(208), Fiat;
    EUR => 2, "EUR", Some(978), Fiat;
    GBP => 2, "GBP", Some(826), Fiat;
    HKD => 2, "HKD", Some(344), Fiat;
    HUF => 2, "HUF", Some(348), Fiat;
    ILS => 2, "ILS", Some(376), Fiat;
    INR => 2, "INR", Some(356), Fiat;
    JPY => 0, "JPY", Some(392), Fiat;
    KRW => 0, "KRW", Some(410), Fiat;
    MXN => 2, "MXN", Some(484), Fiat;
    NOK => 2, "NOK", Some(578), Fiat;
    NZD => 2, "NZD", Some(554), Fiat;
    PLN => 2, "PLN", Some(985), Fiat;
    RUB => 2, "RUB", Some(643), Fiat;
    SAR => 2, "SAR", Some(682), Fiat;
    SEK => 2, "SEK", Some(752), Fiat;
    SGD => 2, "SGD", Some(702), Fiat;
    THB => 2, "THB", Some(764), Fiat;
    TRY => 2, "TRY", Some(949), Fiat;
    TWD => 2, "TWD", Some(901), Fiat;
    USD => 2, "USD", Some(840), Fiat;
    ZAR => 2, "ZAR", Some(710), Fiat;

    // Commodity backed currencies
    XAG => 2, "XAG", Some(961), CommodityBacked;
    XAU => 2, "XAU", Some(959), CommodityBacked;
    XPT => 2, "XPT", Some(962), CommodityBacked;

    // Crypto currencies
    ONEINCH => 8, "1INCH", Some(0), Crypto;
    AAVE => 8, "AAVE", Some(0), Crypto;
    ACA => 8, "ACA", Some(0), Crypto;
    ADA => 6, "ADA", Some(0), Crypto;
    APT => 8, "APT", Some(0), Crypto;
    ARB => 8, "ARB", Some(0), Crypto;
    AVAX => 8, "AVAX", Some(0), Crypto;
    BCH => 8, "BCH", Some(0), Crypto;
    BIO => 8, "BIO", Some(0), Crypto;
    BTC => 8, "BTC", Some(0), Crypto;
    BTTC => 8, "BTTC", Some(0), Crypto;
    BNB => 8, "BNB", Some(0), Crypto;
    BRZ => 8, "BRZ", Some(0), Crypto;
    BSV => 8, "BSV", Some(0), Crypto;
    BUSD => 8, "BUSD", Some(0), Crypto;
    CAKE => 8, "CAKE", Some(0), Crypto;
    CRV => 8, "CRV", Some(0), Crypto;
    DASH => 8, "DASH", Some(0), Crypto;
    DOT => 8, "DOT", Some(0), Crypto;
    DOGE => 8, "DOGE", Some(0), Crypto;
    ENA => 8, "ENA", Some(0), Crypto;
    EOS => 8, "EOS", Some(0), Crypto;
    ETH => 8, "ETH", Some(0), Crypto;
    ETHW => 8, "ETHW", Some(0), Crypto;
    FDUSD => 8, "FDUSD", Some(0), Crypto;
    GWEI => 8, "GWEI", Some(0), Crypto;
    HYPE => 8, "HYPE", Some(0), Crypto;
    JOE => 8, "JOE", Some(0), Crypto;
    LINK => 8, "LINK", Some(0), Crypto;
    LTC => 8, "LTC", Some(0), Crypto;
    LUNA => 8, "LUNA", Some(0), Crypto;
    MAMUSD => 8, "MAMUSD", Some(0), Crypto;
    NBT => 8, "NBT", Some(0), Crypto;
    POL => 8, "POL", Some(0), Crypto;
    PROVE => 8, "PROVE", Some(0), Crypto;
    RLUSD => 8, "RLUSD", Some(0), Crypto;
    SOL => 8, "SOL", Some(0), Crypto;
    SHIB => 8, "SHIB", Some(0), Crypto;
    SUI => 8, "SUI", Some(0), Crypto;
    TON => 8, "TON", Some(0), Crypto;
    TRX => 8, "TRX", Some(0), Crypto;
    TRYB => 8, "TRYB", Some(0), Crypto;
    TUSD => 8, "TUSD", Some(0), Crypto;
    UNI => 8, "UNI", Some(0), Crypto;
    VTC => 8, "VTC", Some(0), Crypto;
    WBTC => 8, "WBTC", Some(0), Crypto;
    WSB => 8, "WSB", Some(0), Crypto;
    XBT => 8, "XBT", Some(0), Crypto;
    XEC => 8, "XEC", Some(0), Crypto;
    XLM => 8, "XLM", Some(0), Crypto;
    XMR => 8, "XMR", Some(0), Crypto;
    USDT => 8, "USDT", Some(0), Crypto;
    XRP => 6, "XRP", Some(0), Crypto;
    XTZ => 6, "XTZ", Some(0), Crypto;
    USDC => 8, "USDC", Some(0), Crypto;
    USDC_POS => 6, "USDC.e", Some(0), Crypto;
    USDG => 8, "USDG", Some(0), Crypto;
    USDP => 8, "USDP", Some(0), Crypto;
    pUSD => 6, "pUSD", Some(0), Crypto;
    ZEC => 8, "ZEC", Some(0), Crypto;
}
