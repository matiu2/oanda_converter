use crate::{client::Client, Error, Result};
pub mod responses;
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
