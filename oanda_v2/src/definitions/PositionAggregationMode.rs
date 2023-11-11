use serde::{Serialize, Deserialize};
/// The way that position values for an Account are calculated
/// and aggregated.
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum PositionAggregationMode {
    /// The Position value or margin for each side (long and
    /// short) of the Position are computed independently and added
    /// together.
    AbsoluteSum,
    /// The Position value or margin for each side (long and short)
    /// of the Position are computed independently. The Position
    /// value or margin chosen is the maximal absolute value of
    /// the two.
    MaximalSide,
    /// The units for each side (long and short) of the Position are
    /// netted together and the resulting value (long or short) is
    /// used to compute the Position value or margin.
    NetSum,
}
