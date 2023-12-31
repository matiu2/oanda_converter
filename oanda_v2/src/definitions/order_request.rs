use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct OrderRequest(String);
impl ToString for OrderRequest {
    fn to_string(&self) -> String {
        self.0
    }
}
impl std::ops::Deref for X {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
