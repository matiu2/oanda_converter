use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct UserAttributes {
    /// The user’s OANDA-assigned user ID.
    user_id: Option<integer>,
    /// The user-provided username.
    username: Option<string>,
    /// The user’s title.
    title: Option<string>,
    /// The user’s name.
    name: Option<string>,
    /// The user’s email address.
    email: Option<string>,
    /// The OANDA division the user belongs to.
    division_abbreviation: Option<string>,
    /// The user’s preferred language.
    language_abbreviation: Option<string>,
    /// The home currency of the Account.
    home_currency: Option<Currency>,
}
