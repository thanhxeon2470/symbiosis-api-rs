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
