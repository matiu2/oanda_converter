//! Generates error.rs for oanda_v2
use crate::{
    error::{Result, Tracer},
    util::{field_name, pretty_doc_string},
};
use error_stack::ResultExt;
use model::definition_docs::{Field, Struct};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Generates a serde derive struct used to talk to the oanda api
pub fn gen_struct(s: &Struct, name: &str) -> Result<TokenStream> {
    let fields = gen_fields(&s.fields)
        .attach_printable_lazy(|| format!("While generating fields for struct {name}"))?;
    let name = Ident::new(name, proc_macro2::Span::call_site());
    Ok(quote! {
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize)]
        struct #name {
            #(#fields)*
        }
    })
}

/// Generates a with a single string inside
pub fn gen_typed_string(name: &str) -> Result<TokenStream> {
    let name = Ident::new(name, proc_macro2::Span::call_site());
    Ok(quote! {
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize, Deref)]
        struct #name (String);

        impl ToString for #name {
            fn to_string(self) -> String {
                self.0
            }
        }
    })
}

fn gen_fields(fields: &[Field]) -> Result<Vec<TokenStream>> {
    fields.iter().map(gen_field).collect()
}

fn gen_field(
    Field {
        name,
        type_name,
        doc_string,
        is_array,
        default,
        required,
    }: &Field,
) -> Result<TokenStream> {
    let name = field_name(name);
    let type_name = Ident::new(type_name, proc_macro2::Span::call_site());
    let doc_string = pretty_doc_string(doc_string)
        .trace()
        .attach_printable("Making the doc string pretty")?;
    let type_name = if *is_array {
        quote! {Vec<#type_name>}
    } else if !*required && default.is_none() {
        quote! {Option<#type_name>}
    } else {
        quote! {#type_name}
    };
    Ok(if let Some(default) = default {
        quote! {
            #(#doc_string)*
            #[serde(default = #default)]
            #name: #type_name,
        }
    } else {
        quote! {
            #(#doc_string)*
            #name: #type_name,
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Tracer;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_gen_struct() -> Result<()> {
        let fields = vec![
            Field {
                name: "field1".to_string(),
                type_name: "String".to_string(),
                doc_string: "Field 1".to_string(),
                is_array: false,
                default: None,
                required: true,
            },
            Field {
                name: "field2".to_string(),
                type_name: "u32".to_string(),
                doc_string: "Field 2".to_string(),
                is_array: true,
                default: None,
                required: false,
            },
            Field {
                name: "name".to_string(),
                type_name: "String".to_string(),
                doc_string: "Don't get too close".to_string(),
                is_array: false,
                default: Some("Mister Fartsy".to_string()),
                required: false,
            },
            Field {
                name: "age".to_string(),
                type_name: "u32".to_string(),
                doc_string: "Optional".to_string(),
                is_array: false,
                default: None,
                required: false,
            },
        ];
        let s = Struct { fields };
        let name = "TestStruct";
        let tokens = gen_struct(&s, name).trace()?;
        let code = crate::util::stream_to_string(&tokens).trace()?;
        println!("{code}");
        assert_eq!(
            code.to_string(),
            indoc! {r#"
                use serde::{Serialize, Deserialize};
                
                #[derive(Serialize, Deserialize)]
                struct TestStruct {
                    /// Field 1
                    field1: String,
                    /// Field 2
                    field2: Vec<u32>,
                    /// Don't get too close
                    #[serde(default = "Mister Fartsy")]
                    name: String,
                    /// Optional
                    age: Option<u32>,
                }
                "#
            }
            .to_string()
        );
        Ok(())
    }

    fn make_field_code(field: Field) -> Result<String> {
        let tokens = gen_field(&field)?;
        // We have to wrap it with a struct so we can produce formatted code
        let tokens = quote! {
            struct Tmp {
                #tokens
            }
        };
        println!("{}", &tokens);
        crate::util::stream_to_string(&tokens).trace()
    }

    #[test]
    fn test_gen_field_simple() -> Result<()> {
        let field = Field {
            name: "name".to_string(),
            type_name: "String".to_string(),
            doc_string: "A very nice field".to_string(),
            is_array: false,
            default: None,
            required: true,
        };
        assert_eq!(
            make_field_code(field).trace()?,
            indoc! {"
            struct Tmp {
                /// A very nice field
                name: String,
            }
            "}
            .to_string()
        );
        Ok(())
    }

    #[test]
    fn test_gen_field_array() -> Result<()> {
        let field = Field {
            name: "names".to_string(),
            type_name: "String".to_string(),
            doc_string: "A vec of names".to_string(),
            is_array: true,
            default: None,
            required: true,
        };
        let code = make_field_code(field).trace()?;
        assert_eq!(
            code,
            indoc! {"
                struct Tmp {
                    /// A vec of names
                    names: Vec<String>,
                }
            "}
        );
        Ok(())
    }

    #[test]
    fn test_gen_field_default() -> Result<()> {
        let field = Field {
            name: "names".to_string(),
            type_name: "String".to_string(),
            doc_string: "A vec of names".to_string(),
            is_array: false,
            default: Some("Mister Joe".to_string()),
            required: true,
        };
        let code = make_field_code(field).trace()?;
        assert_eq!(
            code,
            indoc! {r#"
                struct Tmp {
                    /// A vec of names
                    #[serde(default = "Mister Joe")]
                    names: String,
                }
            "#}
        );
        Ok(())
    }

    #[test]
    fn test_gen_field_optional() -> Result<()> {
        let field = Field {
            name: "optional_name".to_string(),
            type_name: "String".to_string(),
            doc_string: "You don't really need a name".to_string(),
            is_array: false,
            default: None,
            required: false,
        };
        let code = make_field_code(field).trace()?;
        assert_eq!(
            code,
            indoc! {r#"
                struct Tmp {
                    /// You don't really need a name
                    optional_name: Option<String>,
                }
            "#}
        );
        Ok(())
    }

    #[test]
    /// `required` should have no effect if `default` is true
    fn test_gen_field_optional_default() -> Result<()> {
        let field = Field {
            name: "optional_name".to_string(),
            type_name: "String".to_string(),
            doc_string: "You don't really need a name".to_string(),
            is_array: false,
            default: Some("Master Blaster".to_owned()),
            required: false,
        };
        let code = make_field_code(field).trace()?;
        assert_eq!(
            code,
            indoc! {r#"
                struct Tmp {
                    /// You don't really need a name
                    #[serde(default = "Master Blaster")]
                    optional_name: String,
                }
            "#}
        );
        Ok(())
    }

    #[test]
    /// `required` should have no effect if `default` is true
    fn test_gen_field_optional_array() -> Result<()> {
        let field = Field {
            name: "optional_array".to_string(),
            type_name: "String".to_string(),
            doc_string: "Should be an empty array probably 🤷".to_string(),
            is_array: true,
            default: None,
            required: false,
        };
        let code = make_field_code(field).trace()?;
        assert_eq!(
            code,
            indoc! {r#"
                struct Tmp {
                    /// Should be an empty array probably 🤷
                    optional_array: Vec<String>,
                }
            "#}
        );
        Ok(())
    }
}
