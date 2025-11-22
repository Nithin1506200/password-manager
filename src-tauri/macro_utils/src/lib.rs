extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_attribute]
pub fn gen_id(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input struct
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    // Default values
    let mut prefix = String::from("");
    let mut length = 15; // Default length

    // Parse the attribute arguments using a simple approach
    // Convert attribute tokens to a string for easier parsing
    let attr_str = attr.to_string();

    // Parse prefix value
    if let Some(prefix_pos) = attr_str.find("prefix") {
        if let Some(equals_pos) = attr_str[prefix_pos..].find('=') {
            let start_pos = prefix_pos + equals_pos + 1;
            let value_part = &attr_str[start_pos..];

            // Extract the string value (between quotes)
            if let Some(quote_start) = value_part.find('"') {
                if let Some(quote_end) = value_part[quote_start + 1..].find('"') {
                    prefix = value_part[quote_start + 1..quote_start + quote_end + 1].to_string();
                }
            }
        }
    }

    // Parse length value
    if let Some(length_pos) = attr_str.find("length") {
        if let Some(equals_pos) = attr_str[length_pos..].find('=') {
            let start_pos = length_pos + equals_pos + 1;
            let value_part = &attr_str[start_pos..];

            // Extract the number value
            let num_str: String = value_part.chars().take_while(|c| c.is_digit(10)).collect();

            if !num_str.is_empty() {
                if let Ok(num) = num_str.parse::<usize>() {
                    length = num;
                } else {
                    panic!("Failed to parse length value: {}", num_str);
                }
            }
        }
    }

    // Generate the implementation
    let expanded = quote! {
        pub struct #struct_name(String);

        impl #struct_name {
            pub fn new() -> Self {
                Self(#struct_name::generate_id(#length, #prefix))
            }

            fn generate_id(length: usize, prefix: &str) -> String {
                use rand::{distributions::Alphanumeric, Rng};
                let random_str: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(length)
                    .map(|c| c as char)
                    .collect();
                format!("{}{}", prefix, random_str)
                // generates prefix+ random string of len 7
            }

            pub fn to_string(&self) -> String {
                self.0.clone()
            }
        }
    };

    TokenStream::from(expanded)
}
