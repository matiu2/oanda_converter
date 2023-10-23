use crate::client::Client;
struct Account<'a> {
    client: &'a Client,
}
impl<'a> Account<'a> {
    /// Get a list of all Accounts authorized for the provided token.
    pub async fn accounts(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts");
        self.client.get(url).header("Authorization", authorization);
    }
    /// Get the full details for a single Account that a client has access to. Full pending Order, open Trade and open Position representations are provided.
    pub async fn get(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}");
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Get a summary for a single Account that a client has access to.
    pub async fn summary(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/summary");
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Get the list of tradeable instruments for the given Account. The list of tradeable instruments is dependent on the regulatory division that the Account is located in, thus should be the same for all Accounts owned by a single user.
    pub async fn instruments(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/instruments");
        self.client.get(url).header("Authorization", authorization);
    }
    /// Set the client-configurable portions of an Account.
    pub async fn configuration(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/configuration");
        self.client
            .patch(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
    /// Endpoint used to poll an Account for its current state and changes since a specified TransactionID.
    pub async fn changes(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/changes");
        self.client
            .get(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format);
    }
}
