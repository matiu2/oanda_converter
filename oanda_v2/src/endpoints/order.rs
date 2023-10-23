use crate::client::Client;
struct Order<'a> {
    client: &'a Client,
}
impl<'a> Order<'a> {
    pub async fn Orders(&self) -> Result<()> {
        todo!()
    }
    pub async fn Orders(&self) -> Result<()> {
        todo!()
    }
    pub async fn PendingOrders(&self) -> Result<()> {
        todo!()
    }
    pub async fn Get(&self) -> Result<()> {
        todo!()
    }
    pub async fn Put(&self) -> Result<()> {
        todo!()
    }
    pub async fn Cancel(&self) -> Result<()> {
        todo!()
    }
    pub async fn ClientExtensions(&self) -> Result<()> {
        todo!()
    }
}
