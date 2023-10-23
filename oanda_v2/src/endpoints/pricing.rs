use crate::client::Client;
struct Pricing<'a> {
    client: &'a Client,
}
impl<'a> Pricing<'a> {
    pub async fn Latest(&self) -> Result<()> {
        todo!()
    }
    pub async fn Pricing(&self) -> Result<()> {
        todo!()
    }
    pub async fn Stream(&self) -> Result<()> {
        todo!()
    }
    pub async fn Candles(&self) -> Result<()> {
        todo!()
    }
}
