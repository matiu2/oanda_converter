use crate::client::Client;
pub mod responses;
struct Transaction<'a> {
    client: &'a Client,
}
impl<'a> Transaction<'a> {}
