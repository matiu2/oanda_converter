use crate::client::Client;
struct Account<'a> {
    client: &'a Client,
}
impl<'a> Account<'a> {
    pub async fn Accounts(&self) -> Result<()> {
        todo!()
    }
    pub async fn Get(&self) -> Result<()> {
        todo!()
    }
    pub async fn Summary(&self) -> Result<()> {
        todo!()
    }
    pub async fn Instruments(&self) -> Result<()> {
        todo!()
    }
    pub async fn Configuration(&self) -> Result<()> {
        todo!()
    }
    pub async fn Changes(&self) -> Result<()> {
        todo!()
    }
}
