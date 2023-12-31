use serde::{Serialize, Deserialize};
/// The string representation of an Account Identifier.
///
/// Format: “-“-delimited string with format
/// “{siteID}-{divisionID}-{userID}-{accountNumber}”
///
/// Example: 001-011-5838423-001
struct AccountID(String);
impl std::ops::Deref for AccountID {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
