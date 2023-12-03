use crate::client::Client;
pub mod responses;
struct Position<'a> {
    client: &'a Client,
}
impl<'a> Position<'a> {}
