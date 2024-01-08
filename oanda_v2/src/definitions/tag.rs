use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Tag {
    /// The type of the tag.
    r#type: Option<String>,
    /// The name of the tag.
    name: Option<String>,
}
impl Default for Tag {
    fn default() -> Self {
        Self {
            r#type: Default::default(),
            name: Default::default(),
        }
    }
}
