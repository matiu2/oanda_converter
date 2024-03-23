use crate::{
    gen_definition::gen_definition,
    gen_endpoint::{gen_responses_for_call, CallNames},
    gen_mods::gen_mods,
    EasyError, Error, Result,
};
use change_case::{lower_case, pascal_case, snake_case};
use error_stack::ResultExt;
use model::endpoint_docs::{HttpMethod, Response, RestCall};
use model::{Content, Documentation, Endpoint};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use rust_format::{Formatter, PrettyPlease};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use tracing::instrument;

#[derive(Debug)]
pub struct Writer<'a> {
    /// The definition of the Rest client api
    contents: &'a [Content],
    /// Maps a type_name to a uses clause
    type_name_to_uses: HashMap<Cow<'a, str>, HashSet<Location<'a>>>,
    /// Maps a file_name to the uses clauses to insert into it
    mod_lines_by_file: HashMap<String, TokenStream>,
}

impl<'a> Writer<'a> {
    pub fn new(contents: &'a [Content]) -> Writer {
        let type_name_to_uses = gen_map(contents);
        let mod_lines_by_file = gen_all_mods(&type_name_to_uses);
        Writer {
            contents,
            type_name_to_uses,
            mod_lines_by_file,
        }
    }

    /// Given a type_name returns the file it should be written to
    fn type_name_to_file_name(&self, type_name: &str) -> Option<String> {
        self.type_name_to_uses
            .get(&Cow::from(type_name))
            .and_then(|o| o.iter().next())
            .map(|uses| {
                if uses.path.first() == Some(&Cow::from("crate")) {
                    uses.path[1..].join("/") + ".rs"
                } else {
                    uses.path.join("/") + ".rs"
                }
            })
    }

    /// Given a type_name returns the location, (so you can generate uses clauses for it)
    /// If there are multiple uses it just returns the first one
    #[instrument(skip(self))]
    pub fn type_name_to_location(&'a self, type_name: &'a str) -> Option<&'a Location> {
        let result = self
            .type_name_to_uses
            .get(&Cow::from(type_name))
            .and_then(|o| o.iter().next());
        tracing::debug!("Found {result:?}");
        result
    }

    /// Writes the Rest client definitions (json types) in rust
    pub fn write_definitions(&self, base_path: &str) -> Result<()> {
        // Generate all the definitions we need
        let mut definition_mods = Vec::new();
        for definition in self
            .contents
            .iter()
            .flat_map(Content::definitions)
            .flatten()
        {
            let content = gen_definition(definition).attach_printable_lazy(|| {
                format!("Generating definition for {}", definition.name)
            })?;
            let mod_name = change_case::snake_case(&definition.name);
            definition_mods.push(mod_name.clone());
            let filename = format!("{base_path}/definitions/{mod_name}.rs");
            self.stream_to_file(content, &filename)
                .change_context_lazy(|| Error::new(format!("Saving definition to {filename}")))?;
        }
        // Write definitions.rs
        self.stream_to_file(
            gen_mods(definition_mods.as_slice()),
            &format!("{base_path}/definitions.rs"),
        )
        .change_context_lazy(|| Error::new("Generating lib.rs"))?;
        Ok(())
    }

    /// Writes the source code for all the Rest PI endpoints
    /// Returns a list of endpoint names
    pub fn write_endpoints(&self, base_path: &str) -> Result<Vec<&'a str>> {
        // Just list the endpoint names
        let endpoints: Vec<&str> = self
            .contents
            .iter()
            .flat_map(Content::as_endpoint)
            .map(|Endpoint { name, .. }| name.as_str())
            .collect();

        // Generate endpoints.rs
        let tokens = gen_endpoints_mod(&endpoints);
        let filename = format!("{base_path}/endpoints.rs");
        self.stream_to_file(tokens, &filename)
            .change_context_lazy(|| Error::new(format!("Saving endpoint to {filename}")))?;

        // Generate each of the endpoints
        for endpoint in self.contents.iter().flat_map(Content::as_endpoint) {
            let tokens = self
                .gen_endpoint(endpoint)
                .attach_printable_lazy(|| format!("Generating endpoint for {}", endpoint.name))?;
            let filename = format!("{base_path}/endpoints/{}.rs", endpoint.name);
            self.stream_to_file(tokens, &filename)
                .change_context_lazy(|| Error::new(format!("Saving endpoint to {filename}")))?;
            // Generate the responses in a sub module
            let responses_info = get_responses_info(&endpoint.calls)?;
            let tokens = self
                .gen_endpoint_responses(base_path, &endpoint.name, &responses_info)
                .attach_printable_lazy(|| format!("Generating endpoint for {}", endpoint.name))?;
            let filename = format!("{base_path}/endpoints/{}/responses.rs", endpoint.name);
            self.stream_to_file(tokens, &filename)
                .change_context_lazy(|| Error::new(format!("Saving endpoint to {filename}")))?;
        }

        Ok(endpoints)
    }

    /// Writes a token_stream out to a file
    pub fn stream_to_file(&self, stream: TokenStream, path: &str) -> Result<()> {
        // Prepend the token_stream with the "mod" statements
        let stream = if let Some(tokens) = self.mod_lines_by_file.get(path) {
            quote! {#tokens #stream}
        } else {
            stream
        };

        // Work out the all the types it uses, and add their uses clauses

        // Create the dir if it doesn't already exist
        let path = Path::new(path);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)
                .annotate_lazy(|| format!("Unable to create directory \"{dir:#?}\""))?;
        }

        let formatting_result = PrettyPlease::default()
            .format_tokens(stream.clone())
            .annotate_lazy(|| format!("Formatting code for {path:#?}"));
        let formatted_code = match formatting_result {
            Ok(code) => code,
            Err(err) => {
                tracing::error!("Unable to render token stream for {path:#?}. It has been rendered unformatted so you can inspect it: {err:#?}");
                format!("{stream}")
            }
        };

        std::fs::write(path, formatted_code)
            .annotate_lazy(|| format!("Unable to write to \"{path:#?}\""))?;
        Ok(())
    }
}

pub struct ResponsesInfo<'a> {
    pub responses_module_parts: Vec<String>,
    pub struct_prefix: String,
    pub good_response: &'a Response,
    pub bad_responses: Vec<&'a Response>,
    pub token_stream: TokenStream,
}

fn get_responses_info(calls: &[RestCall]) -> Result<HashMap<String, ResponsesInfo>> {
    calls
        .iter()
        .map(|call| {
            let struct_prefix = call.response_struct_prefix()?;
            let responses_module_parts = call.responses_module_parts()?;
            let (good_response, bad_responses) = call.good_and_bad_responses()?;
            let token_stream =
                gen_responses_for_call(&struct_prefix, good_response, bad_responses.as_slice())?;
            Ok((
                call.method_name_as_string()?,
                ResponsesInfo {
                    responses_module_parts,
                    struct_prefix,
                    good_response,
                    bad_responses,
                    token_stream,
                },
            ))
        })
        .collect::<Result<HashMap<String, ResponsesInfo>>>()
}

/// The location of a definition
///
/// Stored as a vec of modules, usually with "crate" at the start.
/// Can be joined with `::` to create
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location<'a> {
    pub path: Vec<Cow<'a, str>>,
}

impl<'a> Location<'a> {
    pub fn new(path: Vec<Cow<'a, str>>) -> Self {
        Self { path }
    }

    /// The whole path as a string with `::` separators
    pub fn as_string(&self) -> String {
        self.path.join("::")
    }

    /// The type name we are defining
    pub fn type_name(&self) -> Option<Cow<'a, str>> {
        self.path.last().cloned()
    }

    /// Returns the token stream to add this location as a uses statement
    pub fn as_uses(&self) -> TokenStream {
        let separators = std::iter::repeat(quote! {::})
            .take(self.path.len() - 1)
            .chain(std::iter::once(quote! {}));
        let tokens: Vec<TokenStream> = self
            .path
            .iter()
            .enumerate()
            .map(|(num, part)| {
                if num == self.path.len() - 1 {
                    // The last part of the path, is the type name so it should be PascalCase
                    pascal_case(part)
                } else {
                    // All the parts leading up to the type name are module names so they are snake case
                    snake_case(part)
                }
            })
            .map(|part| Ident::new(&part, Span::call_site()))
            .map(|ident| quote! {#ident})
            .zip(separators)
            .map(|(ident, separator)| quote!(#ident #separator))
            .collect();
        quote!(use #(#tokens)*;)
    }
}

impl<'a> std::fmt::Display for Location<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.join("::"))
    }
}

/// Generates the map of where each endpoint and definition will be in
/// the source code
///
/// # Returns
///
///  map of `type_name` -> every place that type is declared. The Vec contains each module
fn gen_map<'a>(contents: &'a [Content]) -> HashMap<Cow<'a, str>, HashSet<Location<'a>>> {
    let input = contents
        .iter()
        .map(|c| &c.documentation)
        .flat_map(|d| match d {
            Documentation::Endpoint(Endpoint { name, calls }) => calls
                .iter()
                .map(|call| {
                    Location::new(vec![
                        "crate".into(),
                        "endpoints".into(),
                        snake_case(name).into(),
                        endpoint_method_name(call).unwrap().into(),
                    ])
                })
                .collect::<Vec<Location<'a>>>(),
            Documentation::Definitions { name, definitions } => definitions
                .iter()
                .map(|d| {
                    Location::new(vec![
                        "crate".into(),
                        "definitions".into(),
                        snake_case(name).into(),
                        pascal_case(&d.name).into(),
                    ])
                })
                .collect(),
        })
        .flat_map(|l: Location<'a>| l.type_name().map(|tn| (tn, l)));
    let mut out = HashMap::new();
    for (type_name, location) in input {
        let entry: &mut HashSet<Location<'a>> = out.entry(type_name).or_default();
        entry.insert(location);
    }
    out
}

/// Generate the `mod` statements
///
/// # Returns
///
/// A map of module, and the `mod` statements that they should contain.
pub fn gen_all_mods<'a>(
    map: &'a HashMap<Cow<'a, str>, HashSet<Location<'a>>>,
) -> HashMap<String, TokenStream> {
    // Create a map of all the imports for each file
    let mut to_create: HashMap<String, HashSet<Cow<'a, str>>> = HashMap::new();
    for (file_name, module) in map
        .iter()
        .flat_map(|(_type_name, locations)| locations.iter())
        .flat_map(|location| {
            match location.path.as_slice() {
                [start, a] if start == "crate" => Some(("lib.rs".to_string(), a)),
                [start, inner @ .., holder, last] if start == "crate" => Some((
                    format!("{joined}/{holder}.rs", joined = inner.join("/")),
                    last,
                )),
                // If it's just "crate", or starts with anything other than crate
                // We don't need to generate any `mod xxx` statements.
                _ => None,
            }
        })
    {
        let entry = to_create.entry(file_name).or_default();
        entry.insert(module.clone());
    }
    // Now turn the bits into a proper token stream
    to_create
        .into_iter()
        .map(|(file_name, mods)| (file_name, mods.into_iter().collect::<Vec<_>>()))
        .map(|(file_name, mods)| (file_name, gen_mods(mods.as_slice())))
        .collect()
}

/// Generate all the source code
///
/// # Args
///
/// * `base_path` - The `src` directory where we'll start dumping the generated files
/// * `contents` - The data used to generate the source code
///
/// # Returns
///
/// A map of type_name -> All the places that type is defined
pub fn generate_source(base_path: &str, contents: &[Content]) -> Result<()> {
    // map of type_name to uses_clause
    let writer = Writer::new(contents);

    writer.write_definitions(base_path)?;
    writer.write_endpoints(base_path)?;

    // // We use the mods here
    // let mods = gen_mods(&mods);
    // let lib = quote! {
    //     #mods

    //     pub use error::{Error, Result};
    // };
    // stream_to_file(lib, &format!("{base_path}/lib.rs"))
    //     .change_context_lazy(|| Error::new("Generating lib.rs"))?;
    Ok(Default::default())
}

/// Just generates the src/endpoints.rs
fn gen_endpoints_mod(endpoints: &[&str]) -> TokenStream {
    let mods = endpoints.iter().map(|ep| {
        let ep = Ident::new(ep, proc_macro2::Span::call_site());
        quote!(pub mod #ep;)
    });
    quote!(#(#mods)*)
}

#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => { return Err(error_stack::Report::new($crate::error::Error::Message(format!($($arg),*)))) };
}

/// Takes a name from the yaml and turns it into a nice field name
pub fn field_name(name: &str) -> TokenStream {
    if name == "type" {
        let name = format_ident! { "r#type" };
        quote! { #name }
    } else {
        let name = name.replace('-', "_");
        let name = change_case::snake_case(&name);
        let name = Ident::new(&name, proc_macro2::Span::call_site());
        quote! { #name }
    }
}

/// Returns the method_name without converting it to an Ident
/// Generates a rust name for the method to trigger the Rest api call.
///
/// Takes the path for the rest call. eg. `/v3/accounts/{accountID}/transactions`
/// In most cases, just returns the last segment. eg. `transactions`
/// In cases where the last segment is a variable. eg. `/v3/accounts/{accountid}/transactions/{transactionid}`
/// The method name becomes the HTTP verb, eg `GET` or `POST`
fn endpoint_method_name(ep: &RestCall) -> Result<String> {
    // Get the last path part
    let (_, last_segment) = ep.path.rsplit_once('/').ok_or_else(|| {
        Error::new(format!(
            "Couldn't make method_name out of path with no '/'s: {}",
            &ep.path
        ))
    })?;
    let s = if last_segment.starts_with('{') {
        lower_case(&ep.http_method.to_string())
    } else if ep.http_method == HttpMethod::Post {
        format!("post_{last_segment}")
    } else {
        last_segment.to_string()
    };
    Ok(snake_case(s.as_str()))
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use model::Content;
    use utils::stream_to_string;

    use crate::util::{gen_all_mods, Location};

    use super::Writer;

    #[test]
    fn test_gen_map() {
        let content = std::fs::read_to_string("../content.yaml").unwrap();
        let contents: Vec<Content> = serde_yaml::from_str(&content).unwrap();
        let locations = super::gen_map(&contents);
        println!("{locations:#?}");
        let account_id = locations
            .get(&Cow::from("AccountId"))
            .expect("No AccountId!")
            .iter()
            .next()
            .unwrap();
        let expected = Location {
            path: vec![
                Cow::from("crate"),
                Cow::from("definitions"),
                Cow::from("account"),
                Cow::from("AccountId"),
            ],
        };
        assert_eq!(expected, *account_id);
        // Print the ones that have more than one location
        println!("\n## Double ups\n");
        locations
            .iter()
            .filter(|(_tn, l)| l.len() > 1)
            .for_each(|(tn, l)| println!("{tn}: {l:#?}"));
        // Now test the gen_mods
        let mods = gen_all_mods(&locations);
        for (file_name, stream) in mods {
            let rust = stream_to_string(&stream).unwrap();
            println!("{file_name}: {rust}");
        }
    }

    #[test]
    fn test_type_name_to_file_name() {
        let content = std::fs::read_to_string("../content.yaml").unwrap();
        let contents: Vec<Content> = serde_yaml::from_str(&content).unwrap();
        let writer = Writer::new(contents.as_slice());
        let type_name = "CandlestickResponse";
        let file_name = writer.type_name_to_file_name(type_name);
        println!("File name for {type_name} = {file_name:?}");
        assert_eq!(
            Some("definitions/instrument/CandlestickResponse.rs"),
            file_name.as_deref()
        );
        let location = writer.type_name_to_location(type_name);
        println!("Uses for {type_name} = {location:?}");
        assert_eq!(
            Some("crate::definitions::instrument::CandlestickResponse"),
            location.map(|l| l.to_string()).as_deref()
        );
    }

    #[test]
    fn test_location_as_uses() {
        let loc = super::Location {
            path: vec![
                Cow::from("crate"),
                Cow::from("endpoints"),
                Cow::from("account"),
                Cow::from("responses"),
                Cow::from("get"),
                Cow::from("Get"),
            ],
        };
        let tokens = loc.as_uses();
        let as_text = stream_to_string(&tokens).unwrap();
        println!("{as_text}");
    }
}
