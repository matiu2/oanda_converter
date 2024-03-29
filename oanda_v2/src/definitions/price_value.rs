use serde::{Serialize, Deserialize};
/// The string representation of a Price for a Bucket.
///
/// A decimal number encodes as a string. The amount of
/// precision provided depends on the Instrument.
pub struct PriceValue(String);
impl std::ops::Deref for PriceValue {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
