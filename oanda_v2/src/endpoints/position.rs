use crate::client::Client;
struct Position<'a> {
    client: &'a Client,
}
impl<'a> Position<'a> {
    /// List all Positions for an Account. The Positions returned are for every instrument that has had a position during the lifetime of an the Account.
    pub async fn positions(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/positions");
        self.client.get(url);
    }
    /// List all open Positions for an Account. An open Position is a Position in an Account that currently has a Trade opened for it.
    pub async fn open_positions(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/openPositions");
        self.client.get(url);
    }
    /// Get the details of a single Instrument’s Position in an Account. The Position may by open or not.
    pub async fn get(&self) -> Result<()> {
        let url = self.client.url("/v3/accounts/{accountID}/positions/{instrument}");
        self.client.get(url);
    }
    /// Closeout the open Position for a specific instrument in an Account.
    pub async fn close(&self) -> Result<()> {
        let url = self
            .client
            .url("/v3/accounts/{accountID}/positions/{instrument}/close");
        self.client.put(url);
    }
}
