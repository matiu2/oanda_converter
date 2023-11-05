use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct DividendAdjustmentTransaction {
    /// The Transaction’s Identifier.
    id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    time: Option<DateTime>,
    /// The ID of the user that initiated the creation of the
    /// Transaction.
    user_id: Option<integer>,
    /// The ID of the Account the Transaction was created for.
    account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to.
    /// Transactions in the same batch are applied to the Account
    /// simultaneously.
    batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the
    /// transaction.
    request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to
    /// “DIVIDEND_ADJUSTMENT” for a DividendAdjustmentTransaction.
    #[serde(default = "DIVIDEND_ADJUSTMENT")]
    r#type: TransactionType,
    /// The name of the instrument for the dividendAdjustment
    /// transaction
    instrument: Option<InstrumentName>,
    /// The total dividend adjustment amount paid or collected in
    /// the Account’s home currency for the Account as a result of
    /// applying the DividendAdjustment Transaction. This is the
    /// sum of the dividend adjustments paid/collected for each
    /// OpenTradeDividendAdjustment found within the Transaction.
    dividend_adjustment: Option<AccountUnits>,
    /// The total dividend adjustment amount paid or collected
    /// in the Instrument’s quote currency for the Account as a
    /// result of applying the DividendAdjustment Transaction.
    /// This is the sum of the quote dividend adjustments paid/
    /// collected for each OpenTradeDividendAdjustment found within
    /// the Transaction.
    quote_dividend_adjustment: Option<DecimalNumber>,
    /// The HomeConversionFactors in effect at the time of the
    /// DividendAdjustment.
    home_conversion_factors: Option<HomeConversionFactors>,
    /// The Account balance after applying the DividendAdjustment
    /// Transaction
    account_balance: Option<AccountUnits>,
    /// The dividend adjustment payment/collection details for
    /// each open Trade, within the Account, for which a dividend
    /// adjustment is to be paid or collected.
    open_trade_dividend_adjustments: Vec<OpenTradeDividendAdjustment>,
}
