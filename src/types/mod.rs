use serde::{Deserialize, Serialize};
use serde_utils::*;

/// Symbiosis token.
pub mod token;

/// Symbiosis transaction.
pub mod transaction;

/// Helper for ser/de.
pub mod serde_utils;

/// Symbiosis aggregators trade type.
#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbiosisTradeType {
    /// Dex
    #[default]
    Dex,
    /// 1inch
    #[serde(rename = "1inch")]
    OneInch,
    /// OpenOcean
    #[serde(rename = "open-ocean")]
    OpenOcean,
    /// Wrap
    Wrap,
    /// Izumi
    Izumi,
}

#[cfg(test)]
mod tests {
    use super::SymbiosisTradeType;

    #[test]
    fn token_types_as_str() {
        let items = &[
            (SymbiosisTradeType::Dex, "\"dex\""),
            (SymbiosisTradeType::OneInch, "\"1inch\""),
            (SymbiosisTradeType::OpenOcean, "\"open-ocean\""),
            (SymbiosisTradeType::Wrap, "\"wrap\""),
            (SymbiosisTradeType::Izumi, "\"izumi\""),
        ];

        for (i, s) in items {
            assert_eq!(serde_json::to_string(i).unwrap(), *s);
        }
    }
}
