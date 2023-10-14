use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Deref)]
struct OrderRequest(String);
impl ToString for OrderRequest {
    fn to_string(self) -> String {
        self.0
    }
}
