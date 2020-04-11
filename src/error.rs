#[derive(Debug, snafu::Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("failed to create block mode decryptor: {}", source))]
    CreateBlockMode {
        source: block_modes::InvalidKeyIvLength,
    },

    #[snafu(display("failed to decrypt: {}", source))]
    Decrypt { source: block_modes::BlockModeError },

    #[snafu(display("failed to parse pinentry output ({:?})", out,))]
    FailedToParsePinentry { out: Vec<u8> },

    // no Error impl
    // #[snafu(display("failed to expand with hkdf: {}", source))]
    // HkdfExpand { source: hkdf::InvalidLength },
    #[snafu(display("failed to expand with hkdf"))]
    HkdfExpand,

    // no Error impl
    // #[snafu(display("failed to create hkdf: {}", source))]
    // HkdfFromPrk { source: hkdf::InvalidPrkLength },
    #[snafu(display("failed to create hkdf"))]
    HkdfFromPrk,

    #[snafu(display("invalid base64: {}", source))]
    InvalidBase64 { source: base64::DecodeError },

    #[snafu(display("invalid cipherstring"))]
    InvalidCipherString,

    #[snafu(display("invalid mac"))]
    InvalidMac,

    // no Error impl
    // #[snafu(display("invalid mac key: {}", source))]
    // InvalidMacKey { source: hmac::crypto_mac::InvalidKeyLength },
    #[snafu(display("invalid mac key"))]
    InvalidMacKey,

    #[snafu(display("failed to load config: {}", source))]
    LoadConfig { source: std::io::Error },

    #[snafu(display("failed to load config: {}", source))]
    LoadConfigAsync { source: tokio::io::Error },

    #[snafu(display("failed to load config: {}", source))]
    LoadConfigJson { source: serde_json::Error },

    #[snafu(display("failed to load db: {}", source))]
    LoadDb { source: std::io::Error },

    #[snafu(display("failed to load db: {}", source))]
    LoadDbAsync { source: tokio::io::Error },

    #[snafu(display("failed to load db: {}", source))]
    LoadDbJson { source: serde_json::Error },

    #[snafu(display("error reading pinentry output: {}", source))]
    PinentryReadOutput { source: tokio::io::Error },

    #[snafu(display("error waiting for pinentry to exit: {}", source))]
    PinentryWait { source: tokio::io::Error },

    #[snafu(display("failed to remove db: {}", source))]
    RemoveDb { source: std::io::Error },

    #[snafu(display("error making api request: {}", source))]
    Reqwest { source: reqwest::Error },

    #[snafu(display("failed to save config: {}", source))]
    SaveConfig { source: std::io::Error },

    #[snafu(display("failed to save config: {}", source))]
    SaveConfigJson { source: serde_json::Error },

    #[snafu(display("failed to save db: {}", source))]
    SaveDb { source: std::io::Error },

    #[snafu(display("failed to save db: {}", source))]
    SaveDbAsync { source: tokio::io::Error },

    #[snafu(display("failed to save db: {}", source))]
    SaveDbJson { source: serde_json::Error },

    #[snafu(display("error spawning pinentry: {}", source))]
    Spawn { source: tokio::io::Error },

    #[snafu(display("error writing to pinentry stdin: {}", source))]
    WriteStdin { source: tokio::io::Error },
}

pub type Result<T> = std::result::Result<T, Error>;