use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Tag {
    /// The type of the tag.
    r#type: Option<string>,
    /// The name of the tag.
    name: Option<string>,
}
