use crate::client::Client;
struct Order<'a> {
    client: &'a Client,
}
impl<'a> Order<'a> {
    /// Create an Order for an Account
    pub async fn orders(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/orders");
        self.client.post(url);
    }
    /// Get a list of Orders for an Account
    pub async fn orders(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/orders");
        self.client.get(url);
    }
    /// List all pending Orders in an Account
    pub async fn pending_orders(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/pendingOrders");
        self.client.get(url);
    }
    /// Get details for a single Order in an Account
    pub async fn get(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/orders/{orderSpecifier}");
        self.client.get(url);
    }
    /// Replace an Order in an Account by simultaneously cancelling it and creating a replacement Order
    pub async fn put(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/orders/{orderSpecifier}");
        self.client.put(url);
    }
    /// Cancel a pending Order in an Account
    pub async fn cancel(&self) -> Result<()> {
        let url = self
            .client
            .url("/v3/accounts/{accountID}/orders/{orderSpecifier}/cancel");
        self.client.put(url);
    }
    /// Update the Client Extensions for an Order in an Account. Do not set, modify, or delete clientExtensions if your account is associated with MT4.
    pub async fn client_extensions(&self) -> Result<()> {
        let url = self
            .client
            .url("/v3/accounts/{accountID}/orders/{orderSpecifier}/clientExtensions");
        self.client.put(url);
    }
}
