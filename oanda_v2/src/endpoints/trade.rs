use crate::client::Client;
struct Trade<'a> {
    client: &'a Client,
}
impl<'a> Trade<'a> {
    /// Get a list of Trades for an Account
    pub async fn trades(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        ids: ListOf,
        state: TradeStateFilter,
        instrument: InstrumentName,
        count: Integer,
        before_id: TradeId,
    ) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/trades");
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Get the list of open Trades for an Account
    pub async fn open_trades(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
    ) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/openTrades");
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Get the details of a specific Trade in an Account
    pub async fn get(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        trade_specifier: TradeSpecifier,
    ) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/trades/{tradeSpecifier}");
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Close (partially or fully) a specific open Trade in an Account
    pub async fn close(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        trade_specifier: TradeSpecifier,
    ) -> Result<()> {
        let url = self
            .client
            .url("/v3/accounts/{accountID}/trades/{tradeSpecifier}/close");
        self.client
            .put(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Update the Client Extensions for a Trade. Do not add, update, or delete the Client Extensions if your account is associated with MT4.
    pub async fn client_extensions(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        trade_specifier: TradeSpecifier,
    ) -> Result<()> {
        let url = self
            .client
            .url("/v3/accounts/{accountID}/trades/{tradeSpecifier}/clientExtensions");
        self.client
            .put(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Create, replace and cancel a Trade’s dependent Orders (Take Profit, Stop Loss and Trailing Stop Loss) through the Trade itself
    pub async fn orders(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        trade_specifier: TradeSpecifier,
    ) -> Result<()> {
        let url = self
            .client
            .url("/v3/accounts/{accountID}/trades/{tradeSpecifier}/orders");
        self.client
            .put(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
}
