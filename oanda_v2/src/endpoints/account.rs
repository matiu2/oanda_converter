use crate::{client::Client, Error, Result};
pub mod responses;
struct Account<'a> {
    client: &'a Client,
}
impl<'a> Account<'a> {
    /// Get a list of all Accounts authorized for the provided
    /// token.
    pub async fn accounts(&self, authorization: String) -> Result<()> {
        let url = "/v3/accounts";
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
    /// Get the full details for a single Account that a client has
    /// access to. Full pending Order, open Trade and open Position
    /// representations are provided.
    pub async fn get(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}";
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
    /// Get a summary for a single Account that a client has access
    /// to.
    pub async fn summary(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/summary";
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
    /// Get the list of tradeable instruments for the given Account.
    /// The list of tradeable instruments is dependent on the
    /// regulatory division that the Account is located in, thus
    /// should be the same for all Accounts owned by a single user.
    pub async fn instruments(
        &self,
        authorization: String,
        account_id: AccountId,
        instruments: ListOf,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/instruments";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        let query = [("instruments", instruments)];
        let response = self
            .client
            .get(url)
            .header("Authorization", authorization)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Set the client-configurable portions of an Account.
    pub async fn configuration(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/configuration";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .patch(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Endpoint used to poll an Account for its current state and
    /// changes since a specified TransactionID.
    pub async fn changes(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        since_transaction_id: TransactionId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/changes";
        let url = url.replace("{" + "accountID" + "}");
        let url = self.client.url(url);
        let query = [("sinceTransactionID", since_transaction_id)];
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
}
