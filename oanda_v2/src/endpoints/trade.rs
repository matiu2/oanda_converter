use crate::client::Client;
struct Trade<'a> {
    client: &'a Client,
}
impl<'a> Trade<'a> {
    pub async fn Trades(&self) -> Result<()> {
        todo!()
    }
    pub async fn OpenTrades(&self) -> Result<()> {
        todo!()
    }
    pub async fn Get(&self) -> Result<()> {
        todo!()
    }
    pub async fn Close(&self) -> Result<()> {
        todo!()
    }
    pub async fn ClientExtensions(&self) -> Result<()> {
        todo!()
    }
    pub async fn Orders(&self) -> Result<()> {
        todo!()
    }
}
