use crate::client::Client;
pub mod responses;
struct Order<'a> {
    client: &'a Client,
}
impl<'a> Order<'a> {}
