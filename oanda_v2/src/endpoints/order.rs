use crate::client::Client;
struct Order<'a> {
    client: &'a Client,
}
impl<'a> Order<'a> {
    /// Create an Order for an Account
    pub async fn orders(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/orders";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        self.client
            .post(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Get a list of Orders for an Account
    pub async fn orders(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        ids: ListOf,
        state: OrderStateFilter,
        instrument: InstrumentName,
        count: Integer,
        before_id: OrderId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/orders";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// List all pending Orders in an Account
    pub async fn pending_orders(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/pendingOrders";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Get details for a single Order in an Account
    pub async fn get(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        order_specifier: OrderSpecifier,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/orders/{orderSpecifier}";
        let url = url.replace("{" + "accountID" + "}");
        let url = url.replace("{" + "orderSpecifier" + "}");
        let url = self.client.url(url);
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Replace an Order in an Account by simultaneously cancelling it and creating a replacement Order
    pub async fn put(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        client_request_id: ClientRequestId,
        account_id: AccountId,
        order_specifier: OrderSpecifier,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/orders/{orderSpecifier}";
        let url = url.replace("{" + "accountID" + "}");
        let url = url.replace("{" + "orderSpecifier" + "}");
        let url = self.client.url(url);
        self.client
            .put(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .header("ClientRequestID", client_request_id);
    }
    /// Cancel a pending Order in an Account
    pub async fn cancel(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        client_request_id: ClientRequestId,
        account_id: AccountId,
        order_specifier: OrderSpecifier,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/orders/{orderSpecifier}/cancel";
        let url = url.replace("{" + "accountID" + "}");
        let url = url.replace("{" + "orderSpecifier" + "}");
        let url = self.client.url(url);
        self.client
            .put(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .header("ClientRequestID", client_request_id);
    }
    /// Update the Client Extensions for an Order in an Account. Do not set, modify, or delete clientExtensions if your account is associated with MT4.
    pub async fn client_extensions(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        order_specifier: OrderSpecifier,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/orders/{orderSpecifier}/clientExtensions";
        let url = url.replace("{" + "accountID" + "}");
        let url = url.replace("{" + "orderSpecifier" + "}");
        let url = self.client.url(url);
        self.client
            .put(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
}
