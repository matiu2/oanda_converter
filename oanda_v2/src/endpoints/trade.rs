use crate::client::Client;
struct Trade<'a> {
    client: &'a Client,
}
impl<'a> Trade<'a> {
    /// Get a list of Trades for an Account
    pub async fn trades(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/trades");
        self.client.get(url);
    }
    /// Get the list of open Trades for an Account
    pub async fn open_trades(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/openTrades");
        self.client.get(url);
    }
    /// Get the details of a specific Trade in an Account
    pub async fn get(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/trades/{tradeSpecifier}");
        self.client.get(url);
    }
    /// Close (partially or fully) a specific open Trade in an Account
    pub async fn close(&self) -> Result<()> {
        let url = self
            .client
            .url("/v3/accounts/{accountID}/trades/{tradeSpecifier}/close");
        self.client.put(url);
    }
    /// Update the Client Extensions for a Trade. Do not add, update, or delete the Client Extensions if your account is associated with MT4.
    pub async fn client_extensions(&self) -> Result<()> {
        let url = self
            .client
            .url("/v3/accounts/{accountID}/trades/{tradeSpecifier}/clientExtensions");
        self.client.put(url);
    }
    /// Create, replace and cancel a Trade’s dependent Orders (Take Profit, Stop Loss and Trailing Stop Loss) through the Trade itself
    pub async fn orders(&self) -> Result<()> {
        let url = self
            .client
            .url("/v3/accounts/{accountID}/trades/{tradeSpecifier}/orders");
        self.client.put(url);
    }
}
