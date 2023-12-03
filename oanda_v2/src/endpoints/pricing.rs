use crate::client::Client;
pub mod responses;
struct Pricing<'a> {
    client: &'a Client,
}
impl<'a> Pricing<'a> {}
