use crate::definitions::instrument_name::InstrumentName;
use crate::Result;
use crate::definitions::accept_datetime_format::AcceptDatetimeFormat;
use crate::definitions::order_state_filter::OrderStateFilter;
use crate::client::Client;
use crate::definitions::order_specifier::OrderSpecifier;
use serde::{Serialize, Deserialize};
pub mod responses;
struct Order<'a> {
    client: &'a Client,
}
impl<'a> Order<'a> {
    /// Create an Order for an Account
    pub async fn post_orders(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/orders";
        let url = url.replace("{" + "accountID" + "}", "account_id");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .post(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
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
        let url = url.replace("{" + "accountID" + "}", "account_id");
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
            .get(self.client.start_get(url))
            .await?
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// List all pending Orders in an Account
    pub async fn pending_orders(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/pendingOrders";
        let url = url.replace("{" + "accountID" + "}", "account_id");
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
    /// Get details for a single Order in an Account
    pub async fn get(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        order_specifier: OrderSpecifier,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/orders/{orderSpecifier}";
        let url = url.replace("{" + "accountID" + "}", "account_id");
        let url = url.replace("{" + "orderSpecifier" + "}", "order_specifier");
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
    /// Replace an Order in an Account by simultaneously cancelling
    /// it and creating a replacement Order
    pub async fn put(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        client_request_id: ClientRequestId,
        account_id: AccountId,
        order_specifier: OrderSpecifier,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/orders/{orderSpecifier}";
        let url = url.replace("{" + "accountID" + "}", "account_id");
        let url = url.replace("{" + "orderSpecifier" + "}", "order_specifier");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .put(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .header("ClientRequestID", client_request_id)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
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
        let url = url.replace("{" + "accountID" + "}", "account_id");
        let url = url.replace("{" + "orderSpecifier" + "}", "order_specifier");
        let url = self.client.url(url);
        let query = [];
        let response = self
            .client
            .put(url)
            .header("Authorization", authorization)
            .header("Accept-Datetime-Format", accept_datetime_format)
            .header("ClientRequestID", client_request_id)
            .query(&query)
            .send()
            .await?;
        let status_code = response.status_code();
    }
    /// Update the Client Extensions for an Order in an Account. Do
    /// not set, modify, or delete clientExtensions if your account
    /// is associated with MT4.
    pub async fn client_extensions(
        &self,
        authorization: String,
        accept_datetime_format: AcceptDatetimeFormat,
        account_id: AccountId,
        order_specifier: OrderSpecifier,
    ) -> Result<()> {
        let url = "/v3/accounts/{accountID}/orders/{orderSpecifier}/clientExtensions";
        let url = url.replace("{" + "accountID" + "}", "account_id");
        let url = url.replace("{" + "orderSpecifier" + "}", "order_specifier");
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
