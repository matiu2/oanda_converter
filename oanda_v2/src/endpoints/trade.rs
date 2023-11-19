use crate::client::Client;
/// The list of Trades requested
#[derive(Serialize, Deserialize)]
struct Trades200 {
    /// The list of Trade detail objects
    trades: Vec<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
/// The Account’s list of open Trades is provided
#[derive(Serialize, Deserialize)]
struct OpenTrades200 {
    /// The Account’s list of open Trades
    trades: Vec<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
/// The details for the requested Trade is provided
#[derive(Serialize, Deserialize)]
struct Get200 {
    /// The details of the requested trade
    trade: Option<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
/// The Trade has been closed as requested
#[derive(Serialize, Deserialize)]
struct Close200 {
    /// The MarketOrder Transaction created to close the Trade.
    order_create_transaction: Option<MarketOrderTransaction>,
    /// The OrderFill Transaction that fills the Trade-closing
    /// MarketOrder and closes the Trade.
    order_fill_transaction: Option<OrderFillTransaction>,
    /// The OrderCancel Transaction that immediately cancelled the
    /// Trade-closing MarketOrder.
    order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
#[derive(Serialize, Deserialize)]
struct Close400 {
    /// The MarketOrderReject Transaction that rejects the creation
    /// of the Trade- closing MarketOrder.
    order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
#[derive(Serialize, Deserialize)]
struct Close404 {
    /// The MarketOrderReject Transaction that rejects the creation
    /// of the Trade- closing MarketOrder. Only present if the
    /// Account exists.
    order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The ID of the most recent Transaction created for the
    /// Account. Only present if the Account exists.
    last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request. Only present if the Account exists.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
/// The Trade’s Client Extensions have been updated as
/// requested.
#[derive(Serialize, Deserialize)]
struct ClientExtensions200 {
    /// The Transaction that updates the Trade’s Client Extensions.
    trade_client_extensions_modify_transaction: Option<
        TradeClientExtensionsModifyTransaction,
    >,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
#[derive(Serialize, Deserialize)]
struct ClientExtensions400 {
    /// The Transaction that rejects the modification of the Trade’s
    /// Client Extensions.
    trade_client_extensions_modify_reject_transaction: Option<
        TradeClientExtensionsModifyRejectTransaction,
    >,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
#[derive(Serialize, Deserialize)]
struct ClientExtensions404 {
    /// The Transaction that rejects the modification of the Trade’s
    /// Client Extensions. Only present if the Account exists.
    trade_client_extensions_modify_reject_transaction: Option<
        TradeClientExtensionsModifyRejectTransaction,
    >,
    /// The ID of the most recent Transaction created for the
    /// Account. Only present if the Account exists.
    last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request. Only present if the Account exists.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
/// The Trade’s dependent Orders have been modified as
/// requested.
#[derive(Serialize, Deserialize)]
struct Orders200 {
    /// The Transaction created that cancels the Trade’s existing
    /// Take Profit Order.
    take_profit_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that creates a new Take Profit Order
    /// for the Trade.
    take_profit_order_transaction: Option<TakeProfitOrderTransaction>,
    /// The Transaction created that immediately fills the Trade’s
    /// new Take Profit Order. Only provided if the new Take Profit
    /// Order was immediately filled.
    take_profit_order_fill_transaction: Option<OrderFillTransaction>,
    /// The Transaction created that immediately cancels the Trade’s
    /// new Take Profit Order. Only provided if the new Take Profit
    /// Order was immediately cancelled.
    take_profit_order_created_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that cancels the Trade’s existing
    /// Stop Loss Order.
    stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that creates a new Stop Loss Order
    /// for the Trade.
    stop_loss_order_transaction: Option<StopLossOrderTransaction>,
    /// The Transaction created that immediately fills the Trade’s
    /// new Stop Order. Only provided if the new Stop Loss Order was
    /// immediately filled.
    stop_loss_order_fill_transaction: Option<OrderFillTransaction>,
    /// The Transaction created that immediately cancels the Trade’s
    /// new Stop Loss Order. Only provided if the new Stop Loss
    /// Order was immediately cancelled.
    stop_loss_order_created_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that cancels the Trade’s existing
    /// Trailing Stop Loss Order.
    trailing_stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that creates a new Trailing Stop
    /// Loss Order for the Trade.
    trailing_stop_loss_order_transaction: Option<TrailingStopLossOrderTransaction>,
    /// The Transaction created that cancels the Trade’s existing
    /// Guaranteed Stop Loss Order.
    guaranteed_stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that creates a new Guaranteed Stop
    /// Loss Order for the Trade.
    guaranteed_stop_loss_order_transaction: Option<GuaranteedStopLossOrderTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
#[derive(Serialize, Deserialize)]
struct Orders400 {
    /// An OrderCancelRejectTransaction represents the rejection of
    /// the cancellation of an Order in the client’s Account.
    take_profit_order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// A TakeProfitOrderRejectTransaction represents the rejection
    /// of the creation of a TakeProfit Order.
    take_profit_order_reject_transaction: Option<TakeProfitOrderRejectTransaction>,
    /// An OrderCancelRejectTransaction represents the rejection of
    /// the cancellation of an Order in the client’s Account.
    stop_loss_order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// A StopLossOrderRejectTransaction represents the rejection of
    /// the creation of a StopLoss Order.
    stop_loss_order_reject_transaction: Option<StopLossOrderRejectTransaction>,
    /// An OrderCancelRejectTransaction represents the rejection of
    /// the cancellation of an Order in the client’s Account.
    trailing_stop_loss_order_cancel_reject_transaction: Option<
        OrderCancelRejectTransaction,
    >,
    /// A TrailingStopLossOrderRejectTransaction represents the
    /// rejection of the creation of a TrailingStopLoss Order.
    trailing_stop_loss_order_reject_transaction: Option<
        TrailingStopLossOrderRejectTransaction,
    >,
    /// An OrderCancelRejectTransaction represents the rejection of
    /// the cancellation of an Order in the client’s Account.
    guaranteed_stop_loss_order_cancel_reject_transaction: Option<
        OrderCancelRejectTransaction,
    >,
    /// A GuaranteedStopLossOrderRejectTransaction represents the
    /// rejection of the creation of a GuaranteedStopLoss Order.
    guaranteed_stop_loss_order_reject_transaction: Option<
        GuaranteedStopLossOrderRejectTransaction,
    >,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
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
            .get(url)
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
            .get(url)
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
            .get(url)
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
