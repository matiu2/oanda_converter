use codegen::Scope;
use error_stack::{IntoReport, Report, ResultExt};
use model::defintion_docs::{Content, Definition, EnumItem, Struct, Value};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

mod error;
pub use error::Error;

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

/// Generate the entire code for the Oanda API client
pub fn generate_code(path: &Path, all_content: &[Content]) -> Result<()> {
    // Generate lib.rs
    let mut lib_fn = PathBuf::from(path);
    lib_fn.push("lib.rs");
    let mut lib_file = annotate!(File::create(lib_fn.as_path()), "Creating file {lib_fn:#?}")?;
    let mut lib_code = Scope::new();
    lib_code.raw("mod definitions;");
    annotate!(
        lib_file.write_all(lib_code.to_string().as_bytes()),
        "Writing file {lib_fn:#?}"
    )?;
    Ok(())
}

/// Generates a rust struct or enum from a schema
fn definition(definition: &Definition, scope: &mut Scope) {
    match &definition.value {
        Value::Enum(items) => {
            let mut doc_string = vec![definition.doc_string.clone()];
            let s = scope.new_struct(&definition.name);
            items.iter().for_each(|field| match field {
                EnumItem::ValueDescription { value, description } => todo!(),
                EnumItem::FormattedExample {
                    r#type,
                    format,
                    example,
                } => {
                    doc_string.push(format!("Format: {format}"));
                    doc_string.push(format!("Example: {example}"));
                    assert_eq!(r#type, "string");
                    s.tuple_field("String");
                }
                EnumItem::Example { r#type, example } => todo!(),
                EnumItem::Format { r#type, format } => todo!(),
                EnumItem::JustType { r#type } => todo!(),
            });
            s.derive("Deref");
            s.doc(&doc_string.join("\n"));
        }
        Value::Struct(Struct { fields }) => {
            let s = scope
                .new_struct(&definition.name)
                .doc(&definition.doc_string);
            fields.iter().for_each(|field| {
                s.new_field(&field.name, &field.type_name);
            });
        }
        Value::Empty => {}
    }
    //     scope
    //         .new_struct(definition.name)
    //         .doc(definition.doc_string)

    //         .derive("Debug")
    //         .field("one", "usize")
    //         .field("two", "String")
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::{Error, Result};
    use codegen::Scope;
    use error_stack::{IntoReport, ResultExt};
    use indoc::indoc;
    use model::defintion_docs::Definition;
    use std::{
        env::temp_dir,
        fs::{read_dir, read_to_string, DirEntry},
    };
    use tempfile::tempdir;

    #[test]
    fn test_account_id() {
        let input = indoc! {"
    name: AccountID
    doc_string: The string representation of an Account Identifier.
    value: !Enum
    - !FormattedExample
      type: string
      format: “-“-delimited string with format “{siteID}-{divisionID}-{userID}-{accountNumber}”
      example: 001-011-5838423-001
        "};
        let definition: Definition = serde_yaml::from_str(input).unwrap();

        let mut scope = Scope::new();
        super::definition(&definition, &mut scope);
        println!("Code: {}", scope.to_string());
    }

    #[test]
    fn test_generate_code() -> Result<()> {
        let dir = annotate!(tempdir(), "Creating temp dir")?;
        super::generate_code(dir.path())?;
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
