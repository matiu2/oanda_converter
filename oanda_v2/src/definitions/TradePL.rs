use serde::{Serialize, Deserialize};
/// The classification of TradePLs.
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum TradePL {
    /// An open Trade currently has a positive (profitable)
    /// unrealized P/L, or a closed Trade realized a positive amount
    /// of P/L.
    Positive,
    /// An open Trade currently has a negative (losing) unrealized
    /// P/L, or a closed Trade realized a negative amount of P/L.
    Negative,
    /// An open Trade currently has unrealized P/L of zero (neither
    /// profitable nor losing), or a closed Trade realized a P/L
    /// amount of zero.
    Zero,
}
