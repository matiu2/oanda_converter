use crate::client::Client;
struct Position<'a> {
    client: &'a Client,
}
impl<'a> Position<'a> {
    /// List all Positions for an Account. The Positions returned are for every instrument that has had a position during the lifetime of an the Account.
    pub async fn positions(&self) -> Result<()> {
        todo!()
    }
    /// List all open Positions for an Account. An open Position is a Position in an Account that currently has a Trade opened for it.
    pub async fn open_positions(&self) -> Result<()> {
        todo!()
    }
    /// Get the details of a single Instrument’s Position in an Account. The Position may by open or not.
    pub async fn get(&self) -> Result<()> {
        todo!()
    }
    /// Closeout the open Position for a specific instrument in an Account.
    pub async fn close(&self) -> Result<()> {
        todo!()
    }
}
