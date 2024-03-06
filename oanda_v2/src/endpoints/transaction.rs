use crate::Result;
use crate::client::Client;
use chrono::Utc;
use crate::definitions::accept_datetime_format::AcceptDatetimeFormat;
use chrono::DateTime;
use serde::{Serialize, Deserialize};
pub mod responses;
struct Transaction<'a> {
    client: &'a Client,
}
impl<'a> Transaction<'a> {
    /// Get a list of Transactions pages that satisfy a time-based
    /// Transaction query.
    pub async fn transactions(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        page_size: Integer,
        r#type: ListOf,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/transactions";
        let url = url.replace("{" + "accountID" + "}", "account_id");
        let url = self.client.url(url);
        let query = [
            ("from", from),
            ("to", to),
            ("pageSize", page_size),
            ("type", r#type),
        ];
        let response = self
            .client
            .get(self.client.start_get(url))
            .await?
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Get the details of a single Account Transaction.
    pub async fn get(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        transaction_id: TransactionId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/transactions/{transactionID}";
        let url = url.replace("{" + "accountID" + "}", "account_id");
        let url = url.replace("{" + "transactionID" + "}", "transaction_id");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .get(self.client.start_get(url))
            .await?
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Get a range of Transactions for an Account based on the
    /// Transaction IDs.
    pub async fn idrange(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        from: TransactionId,
        to: TransactionId,
        r#type: ListOf,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/transactions/idrange";
        let url = url.replace("{" + "accountID" + "}", "account_id");
        let url = self.client.url(url);
        let query = [("from", from), ("to", to), ("type", r#type)];
        let response = self
            .client
            .get(self.client.start_get(url))
            .await?
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Get a range of Transactions for an Account starting at (but
    /// not including) a provided Transaction ID.
    pub async fn sinceid(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        id: TransactionId,
        r#type: ListOf,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/transactions/sinceid";
        let url = url.replace("{" + "accountID" + "}", "account_id");
        let url = self.client.url(url);
        let query = [("id", id), ("type", r#type)];
        let response = self
            .client
            .get(self.client.start_get(url))
            .await?
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Get a stream of Transactions for an Account starting from
    /// when the request is made.
    pub async fn stream(
        &self,
        authorization: String,
        account_id: AccountId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/transactions/stream";
        let url = url.replace("{" + "accountID" + "}", "account_id");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .get(self.client.start_get(url))
            .await?
            .header("Authorization", authorization)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
}
