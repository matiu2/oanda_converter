use quote::quote;
use rust_format::{Formatter, PrettyPlease};
use std::fs;

fn main() {
    let source = quote! {
        fn main() {
            println!("Hello World!");
        }
    };

    let formatted_code = PrettyPlease::default().format_tokens(source).unwrap();

    fs::write("output.rs", formatted_code).expect("Unable to write file");
}
