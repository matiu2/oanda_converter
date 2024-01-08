use serde::{Serialize, Deserialize};
/// The string representation of a decimal number.
///
/// A decimal number encoded as a string. The amount of
/// precision provided depends on what the number represents.
struct DecimalNumber(String);
impl std::ops::Deref for DecimalNumber {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
