use convert_case::{Case, Casing};

/// Given a definition_docs struct, generates the code. The actual Rust struct code should already exist;
/// We just fill the fields in
pub fn write_struct_fields(code: &mut codegen::Struct, input: &model::defintion_docs::Struct) {
    for field in &input.fields {
        write_field(code, &field);
    }
}

/// Generates code for a field in a struct
pub fn write_field<'a>(
    scope: &'a mut codegen::Struct,
    field: &'a model::defintion_docs::Field,
) -> &'a mut codegen::Field {
    let basic_type_name = field.type_name.as_str();
    let type_name = match (field.is_array, field.required) {
        (true, true) => format!("Option<Vec<{basic_type_name}>>"),
        (true, false) => format!("Vec<{basic_type_name}>"),
        (false, true) => format!("Option<{basic_type_name}>"),
        (false, false) => basic_type_name.to_string(),
    };
    scope
        .new_field(field.name.to_case(Case::Snake), type_name)
        .doc(&field.doc_string)
    // TODO: Work out how to deal with the field.default value. Perhaps generate a new() that handles it.
}

#[cfg(test)]
mod tests {

    use indoc::indoc;

    #[test]
    fn test_field() {
        let mut scope = codegen::Scope::new();
        let r#struct = scope.new_struct("SomeStruct");
        for (name, (is_array, required)) in
            ('a'..).zip([(false, false), (false, true), (true, false), (true, true)])
        {
            super::write_field(
                r#struct,
                &model::defintion_docs::Field {
                    name: format!("{name}"),
                    type_name: "SomeType".to_string(),
                    doc_string: "Very nice docs".to_string(),
                    is_array,
                    default: Some("Amazing".to_string()),
                    required,
                },
            );
        }
        let code = scope.to_string();
        crate::print_code(&code);
        assert_eq!(
            code,
            indoc!(
                "
struct SomeStruct {
    /// Very nice docs
    a: SomeType,
    /// Very nice docs
    b: Option<SomeType>,
    /// Very nice docs
    c: Vec<SomeType>,
    /// Very nice docs
    d: Option<Vec<SomeType>>,
}"
            )
        );
    }
}
