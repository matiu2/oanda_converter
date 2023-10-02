use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct UserAttributes {
    /// The user’s OANDA-assigned user ID.
    userID: Option<integer>,
    /// The user-provided username.
    username: Option<string>,
    /// The user’s title.
    title: Option<string>,
    /// The user’s name.
    name: Option<string>,
    /// The user’s email address.
    email: Option<string>,
    /// The OANDA division the user belongs to.
    divisionAbbreviation: Option<string>,
    /// The user’s preferred language.
    languageAbbreviation: Option<string>,
    /// The home currency of the Account.
    homeCurrency: Option<Currency>,
}
