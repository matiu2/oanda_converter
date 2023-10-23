use crate::client::Client;
struct Transaction<'a> {
    client: &'a Client,
}
impl<'a> Transaction<'a> {
    /// Get a list of Transactions pages that satisfy a time-based Transaction query.
    pub async fn transactions(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/transactions");
        self.client.get(url);
    }
    /// Get the details of a single Account Transaction.
    pub async fn get(&self) -> Result<()> {
        let url = self
            .client
            .url("/v3/accounts/{accountID}/transactions/{transactionID}");
        self.client.get(url);
    }
    /// Get a range of Transactions for an Account based on the Transaction IDs.
    pub async fn idrange(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/transactions/idrange");
        self.client.get(url);
    }
    /// Get a range of Transactions for an Account starting at (but not including) a provided Transaction ID.
    pub async fn sinceid(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/transactions/sinceid");
        self.client.get(url);
    }
    /// Get a stream of Transactions for an Account starting from when the request is made.
    pub async fn stream(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/transactions/stream");
        self.client.get(url);
    }
}
