use crate::client::Client;
pub mod responses;
struct Account<'a> {
    client: &'a Client,
}
impl<'a> Account<'a> {}
