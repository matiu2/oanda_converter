use serde::{Serialize, Deserialize};
/// The string representation of a quantity of an Account’s
/// home currency.
///
/// A decimal number encoded as a string. The amount of
/// precision provided depends on the Account’s home currency.
pub struct AccountUnits(String);
impl std::ops::Deref for AccountUnits {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
