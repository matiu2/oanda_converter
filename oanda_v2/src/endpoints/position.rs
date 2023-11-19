use crate::client::Client;
/// The Account’s Positions are provided.
#[derive(Serialize, Deserialize)]
struct Positions200 {
    /// The list of Account Positions.
    positions: Vec<Position>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
/// The Account’s open Positions are provided.
#[derive(Serialize, Deserialize)]
struct OpenPositions200 {
    /// The list of open Positions in the Account.
    positions: Vec<Position>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
/// The Position is provided.
#[derive(Serialize, Deserialize)]
struct Get200 {
    /// The requested Position.
    position: Option<Position>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
/// The Position closeout request has been successfully
/// processed.
#[derive(Serialize, Deserialize)]
struct Close200 {
    /// The MarketOrderTransaction created to close the long
    /// Position.
    long_order_create_transaction: Option<MarketOrderTransaction>,
    /// OrderFill Transaction that closes the long Position
    long_order_fill_transaction: Option<OrderFillTransaction>,
    /// OrderCancel Transaction that cancels the MarketOrder created
    /// to close the long Position
    long_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The MarketOrderTransaction created to close the short
    /// Position.
    short_order_create_transaction: Option<MarketOrderTransaction>,
    /// OrderFill Transaction that closes the short Position
    short_order_fill_transaction: Option<OrderFillTransaction>,
    /// OrderCancel Transaction that cancels the MarketOrder created
    /// to close the short Position
    short_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
#[derive(Serialize, Deserialize)]
struct Close400 {
    /// The Transaction created that rejects the creation of a
    /// MarketOrder to close the long Position.
    long_order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The Transaction created that rejects the creation of a
    /// MarketOrder to close the short Position.
    short_order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
#[derive(Serialize, Deserialize)]
struct Close404 {
    /// The Transaction created that rejects the creation of a
    /// MarketOrder to close the long Position. Only present if the
    /// Account exists and a long Position was specified.
    long_order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The Transaction created that rejects the creation of a
    /// MarketOrder to close the short Position. Only present if the
    /// Account exists and a short Position was specified.
    short_order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request. Only present if the Account exists.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account. Only present if the Account exists.
    last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
struct Position<'a> {
    client: &'a Client,
}
impl<'a> Position<'a> {
    /// List all Positions for an Account. The Positions returned
    /// are for every instrument that has had a position during the
    /// lifetime of an the Account.
    pub async fn positions(
        &self,
        authorization: String,
        account_id: AccountId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/positions";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .get(url)
            .header("Authorization", authorization)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// List all open Positions for an Account. An open Position is
    /// a Position in an Account that currently has a Trade opened
    /// for it.
    pub async fn open_positions(
        &self,
        authorization: String,
        account_id: AccountId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/openPositions";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .get(url)
            .header("Authorization", authorization)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Get the details of a single Instrument’s Position in an
    /// Account. The Position may by open or not.
    pub async fn get(
        &self,
        authorization: String,
        account_id: AccountId,
        instrument: InstrumentName,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/positions/{instrument}";
        let url = url.replace("{" + "accountID" + "}");
        let url = url.replace("{" + "instrument" + "}");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .get(url)
            .header("Authorization", authorization)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Closeout the open Position for a specific instrument in
    /// an Account.
    pub async fn close(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        instrument: InstrumentName,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/positions/{instrument}/close";
        let url = url.replace("{" + "accountID" + "}");
        let url = url.replace("{" + "instrument" + "}");
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
