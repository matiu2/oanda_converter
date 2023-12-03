use crate::client::Client;
pub mod responses;
struct Trade<'a> {
    client: &'a Client,
}
impl<'a> Trade<'a> {}
