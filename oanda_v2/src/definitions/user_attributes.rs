use crate::definitions::currency::Currency;
use serde_inline_default::serde_inline_default;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct UserAttributes {
    /// The user’s OANDA-assigned user ID.
    user_id: Option<integer>,
    /// The user-provided username.
    username: Option<String>,
    /// The user’s title.
    title: Option<String>,
    /// The user’s name.
    name: Option<String>,
    /// The user’s email address.
    email: Option<String>,
    /// The OANDA division the user belongs to.
    division_abbreviation: Option<String>,
    /// The user’s preferred language.
    language_abbreviation: Option<String>,
    /// The home currency of the Account.
    home_currency: Option<Currency>,
}
impl Default for UserAttributes {
    fn default() -> Self {
        Self {
            user_id: Default::default(),
            username: Default::default(),
            title: Default::default(),
            name: Default::default(),
            email: Default::default(),
            division_abbreviation: Default::default(),
            language_abbreviation: Default::default(),
            home_currency: Default::default(),
        }
    }
}
