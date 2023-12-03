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
#[derive(Debug)]
pub enum Error {
    E400(Configuration400),
    E403(Configuration403),
}
