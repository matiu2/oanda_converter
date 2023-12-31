use error_stack::ResultExt;
use model::Content;
use std::fs::read_to_string;
use uses_inserter::insert_uses_clauses;
use uses_inserter::ModName;
use writer::{util::generate_source, EasyError, Error};

pub type Result<T> = error_stack::Result<T, Error>;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let base_path = "src";
    // You will have already run 'serialize_all' and generated a content.yaml. Now we'll read it in
    let yaml = read_to_string("content.yaml")
        .or_else(|_| read_to_string("../content.yaml"))
        .annotate("Opening content.yaml")?;
    let content: Vec<Content> = serde_yaml::from_str(&yaml).annotate("Reading in content.yaml")?;
    generate_source(base_path, content.as_slice()).attach_printable("Generating the source")?;
    let mod_name = ModName::new("src").add_part("lib");
    let files_to_ignore = ["host", "error", "lib", "client"]
        .into_iter()
        .map(|s| ModName::new(base_path).add_part(s))
        .collect();
    let known_sources = vec![
        ("DateTime", ModName::new("").add_part("chrono")),
        ("Utc", ModName::new("").add_part("chrono")),
        ("Serialize", ModName::new("").add_part("serde")),
        ("Deserialize", ModName::new("").add_part("serde")),
    ]
    .into_iter()
    .collect();
    insert_uses_clauses(mod_name, &files_to_ignore, &known_sources)
        .change_context_lazy(|| Error::new("Inserting uses clauses"))
}
