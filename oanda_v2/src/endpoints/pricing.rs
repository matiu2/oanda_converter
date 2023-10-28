use crate::client::Client;
struct Pricing<'a> {
    client: &'a Client,
}
impl<'a> Pricing<'a> {
    /// Get dancing bears and most recently completed candles within an Account for specified combinations of instrument, granularity, and price component.
    pub async fn latest(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        candle_specifications: ListOf,
        units: DecimalNumber,
        smooth: Boolean,
        daily_alignment: Integer,
        alignment_timezone: String,
        weekly_alignment: WeeklyAlignment,
    ) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/candles/latest");
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Get pricing information for a specified list of Instruments within an Account.
    pub async fn pricing(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        instruments: ListOf,
        since: DateTime,
        include_units_available: Boolean,
        include_home_conversions: Boolean,
    ) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/pricing");
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Get a stream of Account Prices starting from when the request is made.
    /// This pricing stream does not include every single price created for the Account, but instead will provide at most 4 prices per second (every 250 milliseconds) for each instrument being requested.
    /// If more than one price is created for an instrument during the 250 millisecond window, only the price in effect at the end of the window is sent. This means that during periods of rapid price movement, subscribers to this stream will not be sent every price.
    /// Pricing windows for different connections to the price stream are not all aligned in the same way (i.e. they are not all aligned to the top of the second). This means that during periods of rapid price movement, different subscribers may observe different prices depending on their alignment.
    pub async fn stream(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        instruments: ListOf,
        snapshot: Boolean,
        include_home_conversions: Boolean,
    ) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/pricing/stream");
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Fetch candlestick data for an instrument.
    pub async fn candles(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        instrument: InstrumentName,
        price: PricingComponent,
        granularity: CandlestickGranularity,
        count: Integer,
        from: DateTime,
        to: DateTime,
        smooth: Boolean,
        include_first: Boolean,
        daily_alignment: Integer,
        alignment_timezone: String,
        weekly_alignment: WeeklyAlignment,
        units: DecimalNumber,
    ) -> Result<()> {
        let url = self
            .client
            .url("/v3/accounts/{accountID}/instruments/{instrument}/candles");
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
}
