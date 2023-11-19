use crate::client::Client;
/// The list of authorized Accounts has been provided.
#[derive(Serialize, Deserialize)]
struct Accounts200 {
    /// The list of Accounts the client is authorized to access and
    /// their associated properties.
    accounts: Vec<AccountProperties>,
}
/// The full Account details are provided
#[derive(Serialize, Deserialize)]
struct Get200 {
    /// The full details of the requested Account.
    account: Option<Account>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
}
/// The Account summary are provided
#[derive(Serialize, Deserialize)]
struct Summary200 {
    /// The summary of the requested Account.
    account: Option<AccountSummary>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
}
/// The list of tradeable instruments for the Account has been
/// provided.
#[derive(Serialize, Deserialize)]
struct Instruments200 {
    /// The requested list of instruments.
    instruments: Vec<Instrument>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
}
/// The Account was configured successfully.
#[derive(Serialize, Deserialize)]
struct Configuration200 {
    /// The transaction that configures the Account.
    client_configure_transaction: Option<ClientConfigureTransaction>,
    /// The ID of the last Transaction created for the Account.
    last_transaction_id: Option<TransactionID>,
}
#[derive(Serialize, Deserialize)]
struct Configuration400 {
    /// The transaction that rejects the configuration of the
    /// Account.
    client_configure_reject_transaction: Option<ClientConfigureRejectTransaction>,
    /// The ID of the last Transaction created for the Account.
    last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
#[derive(Serialize, Deserialize)]
struct Configuration403 {
    /// The transaction that rejects the configuration of the
    /// Account.
    client_configure_reject_transaction: Option<ClientConfigureRejectTransaction>,
    /// The ID of the last Transaction created for the Account.
    last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
/// The Account state and changes are provided.
#[derive(Serialize, Deserialize)]
struct Changes200 {
    /// The changes to the Account’s Orders, Trades and Positions
    /// since the specified Transaction ID. Only provided if the
    /// sinceTransactionID is supplied to the poll request.
    changes: Option<AccountChanges>,
    /// The Account’s current price-dependent state.
    state: Option<AccountChangesState>,
    /// The ID of the last Transaction created for the Account.
    /// This Transaction ID should be used for future poll requests,
    /// as the client has already observed all changes up to and
    /// including it.
    last_transaction_id: Option<TransactionID>,
}
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
