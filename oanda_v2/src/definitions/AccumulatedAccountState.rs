use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct AccumulatedAccountState {
    /// The current balance of the account.
    balance: Option<AccountUnits>,
    /// The total profit/loss realized over the lifetime of the Account.
    pl: Option<AccountUnits>,
    /// The total realized profit/loss for the account since it was last reset by the client.
    resettable_pl: Option<AccountUnits>,
    /// The total amount of financing paid/collected over the lifetime of the account.
    financing: Option<AccountUnits>,
    /// The total amount of commission paid over the lifetime of the Account.
    commission: Option<AccountUnits>,
    /// The total amount of dividend adjustment paid over the lifetime of the Account in the Account’s home currency.
    dividend_adjustment: Option<AccountUnits>,
    /// The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop Loss Orders.
    guaranteed_execution_fees: Option<AccountUnits>,
    /// The date/time when the Account entered a margin call state. Only provided if the Account is in a margin call.
    margin_call_enter_time: Option<DateTime>,
    /// The number of times that the Account’s current margin call was extended.
    margin_call_extension_count: Option<integer>,
    /// The date/time of the Account’s last margin call extension.
    last_margin_call_extension_time: Option<DateTime>,
}
