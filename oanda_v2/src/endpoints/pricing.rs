use crate::client::Client;
struct Pricing<'a> {
    client: &'a Client,
}
impl<'a> Pricing<'a> {
    /// Get dancing bears and most recently completed candles
    /// within an Account for specified combinations of instrument,
    /// granularity, and price component.
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
        let url = "/v3/accounts/{accountID}/candles/latest";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        let query = [
            ("candleSpecifications", candle_specifications),
            ("units", units),
            ("smooth", smooth),
            ("dailyAlignment", daily_alignment),
            ("alignmentTimezone", alignment_timezone),
            ("weeklyAlignment", weekly_alignment),
        ];
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query);
    }
    /// Get pricing information for a specified list of Instruments
    /// within an Account.
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
        let url = "/v3/accounts/{accountID}/pricing";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        let query = [
            ("instruments", instruments),
            ("since", since),
            ("includeUnitsAvailable", include_units_available),
            ("includeHomeConversions", include_home_conversions),
        ];
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query);
    }
    /// Get a stream of Account Prices starting from when the
    /// request is made.
    /// This pricing stream does not include every single price
    /// created for the Account, but instead will provide at most
    /// 4 prices per second (every 250 milliseconds) for each
    /// instrument being requested.
    /// If more than one price is created for an instrument during
    /// the 250 millisecond window, only the price in effect at the
    /// end of the window is sent. This means that during periods of
    /// rapid price movement, subscribers to this stream will not be
    /// sent every price.
    /// Pricing windows for different connections to the price
    /// stream are not all aligned in the same way (i.e. they
    /// are not all aligned to the top of the second). This means
    /// that during periods of rapid price movement, different
    /// subscribers may observe different prices depending on their
    /// alignment.
    pub async fn stream(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        instruments: ListOf,
        snapshot: Boolean,
        include_home_conversions: Boolean,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/pricing/stream";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        let query = [
            ("instruments", instruments),
            ("snapshot", snapshot),
            ("includeHomeConversions", include_home_conversions),
        ];
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query);
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
        let url = "/v3/accounts/{accountID}/instruments/{instrument}/candles";
        let url = url.replace("{" + "accountID" + "}");
        let url = url.replace("{" + "instrument" + "}");
        let url = self.client.url(url);
        let query = [
            ("price", price),
            ("granularity", granularity),
            ("count", count),
            ("from", from),
            ("to", to),
            ("smooth", smooth),
            ("includeFirst", include_first),
            ("dailyAlignment", daily_alignment),
            ("alignmentTimezone", alignment_timezone),
            ("weeklyAlignment", weekly_alignment),
            ("units", units),
        ];
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query);
    }
}
