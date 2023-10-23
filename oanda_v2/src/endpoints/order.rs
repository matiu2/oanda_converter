use crate::client::Client;
struct Order<'a> {
    client: &'a Client,
}
impl<'a> Order<'a> {
    /// Create an Order for an Account
    pub async fn orders(&self) -> Result<()> {
        todo!()
    }
    /// Get a list of Orders for an Account
    pub async fn orders(&self) -> Result<()> {
        todo!()
    }
    /// List all pending Orders in an Account
    pub async fn pending_orders(&self) -> Result<()> {
        todo!()
    }
    /// Get details for a single Order in an Account
    pub async fn get(&self) -> Result<()> {
        todo!()
    }
    /// Replace an Order in an Account by simultaneously cancelling it and creating a replacement Order
    pub async fn put(&self) -> Result<()> {
        todo!()
    }
    /// Cancel a pending Order in an Account
    pub async fn cancel(&self) -> Result<()> {
        todo!()
    }
    /// Update the Client Extensions for an Order in an Account. Do not set, modify, or delete clientExtensions if your account is associated with MT4.
    pub async fn client_extensions(&self) -> Result<()> {
        todo!()
    }
}
