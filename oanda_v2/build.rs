use error_stack::ResultExt;
use model::Content;
use std::fs::read_to_string;
use std::io::Write;
use uses_inserter::insert_uses_clauses;
use uses_inserter::ModName;
use walkdir::WalkDir;
use writer::{util::generate_source, EasyError, Error};

pub type Result<T> = error_stack::Result<T, Error>;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_file(true)
                .with_line_number(true),
        )
        .init();
    let base_path = "src";
    // You will have already run 'serialize_all' and generated a content.yaml. Now we'll read it in
    let yaml = read_to_string("content.yaml")
        .or_else(|_| read_to_string("../content.yaml"))
        .annotate("Opening content.yaml")?;
    let content: Vec<Content> = serde_yaml::from_str(&yaml).annotate("Reading in content.yaml")?;
    generate_source(base_path, content.as_slice()).attach_printable("Generating the source")?;

    // Search and replace DateTime with DateTime
    for entry in WalkDir::new("src") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.path().extension().unwrap_or_default() == "rs" {
            let content = std::fs::read_to_string(entry.path())
                .annotate_lazy(|| format!("Reading file {entry:#?}"))?;
            let replaced_content = content
                .replace("DateTime", "DateTime<Utc>")
                // This may be run on code that already contains <Utc>
                // - this next line stops us continually appending <Utc>
                .replace("DateTime<Utc><Utc>", "DateTime<Utc>")
                .replace("use chrono::DateTime<Utc>", "use chrono::DateTime")
                .replace("Boolean", "bool");
            let mut file = std::fs::File::create(entry.path())
                .annotate_lazy(|| format!("Writing file {entry:#?}"))?;
            file.write_all(replaced_content.as_bytes())
                .annotate_lazy(|| format!("Writing file {entry:#?}"))?;
        }
    }

    // Insert uses clauses
    let mod_name = ModName::new("src").add_part("lib");
    let files_to_ignore = ["host", "error", "lib", "client"]
        .into_iter()
        .map(|s| ModName::new(base_path).add_part("lib").add_part(s))
        .collect();
    let known_sources = vec![
        ("DateTime", ModName::new("").add_part("chrono")),
        ("Result", ModName::new("").add_part("crate")),
        ("Utc", ModName::new("").add_part("chrono")),
        ("Serialize", ModName::new("").add_part("serde")),
        ("Deserialize", ModName::new("").add_part("serde")),
        (
            "serde_inline_default",
            ModName::new("").add_part("serde_inline_default"),
        ),
    ]
    .into_iter()
    .collect();
    insert_uses_clauses(mod_name, &files_to_ignore, &known_sources)
        .change_context_lazy(|| Error::new("Inserting uses clauses"))
    // panic!("Stopping here to see output messages");
}
