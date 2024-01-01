use definitions::currency::Currency;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
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
        use Default::default;
        Self {
            user_id: default(),
            username: default(),
            title: default(),
            name: default(),
            email: default(),
            division_abbreviation: default(),
            language_abbreviation: default(),
            home_currency: default(),
        }
    }
}
