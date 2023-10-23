use crate::client::Client;
struct Transaction<'a> {
    client: &'a Client,
}
impl<'a> Transaction<'a> {
    /// Get a list of Transactions pages that satisfy a time-based Transaction query.
    pub async fn transactions(&self) -> Result<()> {
        todo!()
    }
    /// Get the details of a single Account Transaction.
    pub async fn get(&self) -> Result<()> {
        todo!()
    }
    /// Get a range of Transactions for an Account based on the Transaction IDs.
    pub async fn idrange(&self) -> Result<()> {
        todo!()
    }
    /// Get a range of Transactions for an Account starting at (but not including) a provided Transaction ID.
    pub async fn sinceid(&self) -> Result<()> {
        todo!()
    }
    /// Get a stream of Transactions for an Account starting from when the request is made.
    pub async fn stream(&self) -> Result<()> {
        todo!()
    }
}
