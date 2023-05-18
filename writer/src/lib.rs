use codegen::Scope;
use definitions::create_definition;
use endpoints::create_endpoint;
use error_stack::{IntoReport, Report, ResultExt};
use model::Content;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

mod error;
pub use error::Error;

mod definitions;
mod endpoints;

type Result<T> = std::result::Result<T, Report<Error>>;

#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        error_stack::bail!(Error::new(format!($($arg)*)))
    };
}

#[macro_export]
macro_rules! report {
    ($($arg:tt)*) => {
        error_stack::report!(Error::new(format!($($arg)*)))
    };
}

#[macro_export]
macro_rules! annotate {
    ($result:expr, $fmt:expr) => {
        {
            $result.into_report().change_context(Error::new(format!($fmt)))
        }
    };
   ($result:expr, $fmt:expr, $($arg:expr),*) => {
        {
            $result.into_report().change_context(Error::new(format!($fmt, $($arg),*)))
        }
    };
}

/// Create a new directory under `path`
fn add_dir(path: &Path, new_dir_name: &str) -> Result<PathBuf> {
    let mut new_path = PathBuf::from(path);
    new_path.push(new_dir_name);
    annotate!(
        std::fs::create_dir_all(new_path.as_path()),
        "Creating dir {new_path:#?}"
    )?;
    Ok(new_path)
}

/// Generate the entire code for the Oanda API client
pub fn generate_code(path: &Path, all_content: &[Content]) -> Result<()> {
    // Generate lib.rs
    let mut lib_fn = PathBuf::from(path);
    lib_fn.push("lib.rs");
    let mut lib_file = annotate!(File::create(lib_fn.as_path()), "Creating file {lib_fn:#?}")?;
    let mut lib_code = Scope::new();
    lib_code.raw("mod definitions;");
    lib_code.raw("mod endpoints;");
    annotate!(
        lib_file.write_all(lib_code.to_string().as_bytes()),
        "Writing file {lib_fn:#?}"
    )?;
    // Generate the definitions module
    let endpoints_dir = add_dir(path, "endpoints")?;
    let definitions_dir = add_dir(path, "definitions")?;
    // Generate the content for each entry
    for content in all_content {
        match &content.documentation {
            model::Documentation::Endpoint(rest_calls) => {
                create_endpoint(&endpoints_dir, rest_calls.as_slice())?
            }
            model::Documentation::Definitions(definitions) => {
                create_definition(&definitions_dir, definitions.as_slice())?
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Error, Result};
    use error_stack::{IntoReport, ResultExt};
    use model::Content;
    use std::fs::{read_dir, read_to_string, DirEntry};
    use tempfile::tempdir;

    #[test]
    fn test_generate_code() -> Result<()> {
        let dir = annotate!(tempdir(), "Creating temp dir")?;
        // Load all the content from the scraped yaml file
        let content_yaml = annotate!(
            read_to_string("../serialize_all/content.yaml"),
            "While reading content.yaml"
        )?;
        let all_content: Vec<Content> = annotate!(
            serde_yaml::from_str(&content_yaml),
            "While parsing content.yaml"
        )?;

        super::generate_code(dir.path(), all_content.as_slice())?;
        let list = annotate!(read_dir(dir.path()), "While reading {dir:#?}")?;
        let list = list
            .map(|result| annotate!(result, "While listing {dir:#?}"))
            .collect::<Result<Vec<DirEntry>>>()?;
        println!("{list:#?}");
        let lib = list
            .get(0)
            .ok_or_else(|| Error::new(format!("Expcted at least one file name in {list:#?}")))?;
        // .ok_or_else(|| Error::new(format!("There should be a lib.rs in {dir:#?}")))
        let contents = annotate!(
            read_to_string(lib.path()),
            "While reading from generate lib.rs: {lib:#?}"
        )?;
        println!("{contents}");
        Ok(())
    }
}
