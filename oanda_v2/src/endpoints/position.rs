use crate::client::Client;
struct Position<'a> {
    client: &'a Client,
}
impl<'a> Position<'a> {
    pub async fn Positions(&self) -> Result<()> {
        todo!()
    }
    pub async fn OpenPositions(&self) -> Result<()> {
        todo!()
    }
    pub async fn Get(&self) -> Result<()> {
        todo!()
    }
    pub async fn Close(&self) -> Result<()> {
        todo!()
    }
}
