use definitions::guaranteed_stop_loss_order_mode_for_instrument::GuaranteedStopLossOrderModeForInstrument;
use definitions::decimal_number::DecimalNumber;
use definitions::instrument_commission::InstrumentCommission;
use definitions::instrument_type::InstrumentType;
use definitions::guaranteed_stop_loss_order_level_restriction::GuaranteedStopLossOrderLevelRestriction;
use definitions::instrument_financing::InstrumentFinancing;
use definitions::tag::Tag;
use definitions::instrument_name::InstrumentName;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Instrument {
    /// The name of the Instrument
    name: Option<InstrumentName>,
    /// The type of the Instrument
    r#type: Option<InstrumentType>,
    /// The display name of the Instrument
    display_name: Option<String>,
    /// The location of the “pip” for this instrument. The decimal
    /// position of the pip in this Instrument’s price can be
    /// found at 10 ^ pipLocation (e.g. -4 pipLocation results in a
    /// decimal pip position of 10 ^ -4 = 0.0001).
    pip_location: Option<integer>,
    /// The number of decimal places that should be used to display
    /// prices for this instrument. (e.g. a displayPrecision of 5
    /// would result in a price of “1” being displayed as “1.00000”)
    display_precision: Option<integer>,
    /// The amount of decimal places that may be provided when
    /// specifying the number of units traded for this instrument.
    trade_units_precision: Option<integer>,
    /// The smallest number of units allowed to be traded for this
    /// instrument.
    minimum_trade_size: Option<DecimalNumber>,
    /// The maximum trailing stop distance allowed for a trailing
    /// stop loss created for this instrument. Specified in price
    /// units.
    maximum_trailing_stop_distance: Option<DecimalNumber>,
    /// The minimum distance allowed between the Trade’s fill price
    /// and the configured price for guaranteed Stop Loss Orders
    /// created for this instrument. Specified in price units.
    minimum_guaranteed_stop_loss_distance: Option<DecimalNumber>,
    /// The minimum trailing stop distance allowed for a trailing
    /// stop loss created for this instrument. Specified in price
    /// units.
    minimum_trailing_stop_distance: Option<DecimalNumber>,
    /// The maximum position size allowed for this instrument.
    /// Specified in units.
    maximum_position_size: Option<DecimalNumber>,
    /// The maximum units allowed for an Order placed for this
    /// instrument. Specified in units.
    maximum_order_units: Option<DecimalNumber>,
    /// The margin rate for this instrument.
    margin_rate: Option<DecimalNumber>,
    /// The commission structure for this instrument.
    commission: Option<InstrumentCommission>,
    /// The current Guaranteed Stop Loss Order mode of the Account
    /// for this Instrument.
    guaranteed_stop_loss_order_mode: Option<GuaranteedStopLossOrderModeForInstrument>,
    /// The amount that is charged to the account if a guaranteed
    /// Stop Loss Order is triggered and filled. The value
    /// is in price units and is charged for each unit of the
    /// Trade. This field will only be present if the Account’s
    /// guaranteedStopLossOrderMode for this Instrument is not
    /// ‘DISABLED’.
    guaranteed_stop_loss_order_execution_premium: Option<DecimalNumber>,
    /// The guaranteed Stop Loss Order level restriction for this
    /// instrument. This field will only be present if the Account’s
    /// guaranteedStopLossOrderMode for this Instrument is not
    /// ‘DISABLED’.
    guaranteed_stop_loss_order_level_restriction: Option<
        GuaranteedStopLossOrderLevelRestriction,
    >,
    /// Financing data for this instrument.
    financing: Option<InstrumentFinancing>,
    /// The tags associated with this instrument.
    tags: Vec<Tag>,
}
impl Default for Instrument {
    fn default() -> Self {
        use Default::default;
        Self {
            name: default(),
            r#type: default(),
            display_name: default(),
            pip_location: default(),
            display_precision: default(),
            trade_units_precision: default(),
            minimum_trade_size: default(),
            maximum_trailing_stop_distance: default(),
            minimum_guaranteed_stop_loss_distance: default(),
            minimum_trailing_stop_distance: default(),
            maximum_position_size: default(),
            maximum_order_units: default(),
            margin_rate: default(),
            commission: default(),
            guaranteed_stop_loss_order_mode: default(),
            guaranteed_stop_loss_order_execution_premium: default(),
            guaranteed_stop_loss_order_level_restriction: default(),
            financing: default(),
            tags: default(),
        }
    }
}
