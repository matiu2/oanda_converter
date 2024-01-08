use definitions::accept_datetime_format::AcceptDatetimeFormat;
use definitions::trade_state_filter::TradeStateFilter;
use definitions::instrument_name::InstrumentName;
use definitions::trade_specifier::TradeSpecifier;
use crate::{client::Client, Error, Result};
pub mod responses;
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
        let url = "/v3/accounts/{accountID}/trades";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        let query = [
            ("ids", ids),
            ("state", state),
            ("instrument", instrument),
            ("count", count),
            ("beforeID", before_id),
        ];
        let response = self
            .client
            .get(self.client.start_get(url))
            .await?
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Get the list of open Trades for an Account
    pub async fn open_trades(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/openTrades";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .get(self.client.start_get(url))
            .await?
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Get the details of a specific Trade in an Account
    pub async fn get(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        trade_specifier: TradeSpecifier,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/trades/{tradeSpecifier}";
        let url = url.replace("{" + "accountID" + "}");
        let url = url.replace("{" + "tradeSpecifier" + "}");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .get(self.client.start_get(url))
            .await?
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Close (partially or fully) a specific open Trade in an
    /// Account
    pub async fn close(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        trade_specifier: TradeSpecifier,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/trades/{tradeSpecifier}/close";
        let url = url.replace("{" + "accountID" + "}");
        let url = url.replace("{" + "tradeSpecifier" + "}");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .put(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Update the Client Extensions for a Trade. Do not add,
    /// update, or delete the Client Extensions if your account is
    /// associated with MT4.
    pub async fn client_extensions(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        trade_specifier: TradeSpecifier,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/trades/{tradeSpecifier}/clientExtensions";
        let url = url.replace("{" + "accountID" + "}");
        let url = url.replace("{" + "tradeSpecifier" + "}");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .put(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Create, replace and cancel a Trade’s dependent Orders (Take
    /// Profit, Stop Loss and Trailing Stop Loss) through the Trade
    /// itself
    pub async fn orders(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        trade_specifier: TradeSpecifier,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/trades/{tradeSpecifier}/orders";
        let url = url.replace("{" + "accountID" + "}");
        let url = url.replace("{" + "tradeSpecifier" + "}");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .put(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
}
