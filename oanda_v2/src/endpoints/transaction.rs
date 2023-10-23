use crate::client::Client;
struct Transaction<'a> {
    client: &'a Client,
}
impl<'a> Transaction<'a> {
    pub async fn Transactions(&self) -> Result<()> {
        todo!()
    }
    pub async fn Get(&self) -> Result<()> {
        todo!()
    }
    pub async fn Idrange(&self) -> Result<()> {
        todo!()
    }
    pub async fn Sinceid(&self) -> Result<()> {
        todo!()
    }
    pub async fn Stream(&self) -> Result<()> {
        todo!()
    }
}
