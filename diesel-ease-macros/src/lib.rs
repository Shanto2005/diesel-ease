// #![allow(unused, dead_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::*; // TODO: Later import only the necessary parts.

#[proc_macro_attribute]
pub fn auto_load(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;

    let name_lower = name.to_string().to_lowercase();

    let name_lower = format!("{}s", name_lower);

    let name_lower = Ident::new(&name_lower, name.span());

    let args = parse_macro_input!(args as Ident);

    quote! {
            #input

            impl #name {
                pub fn load(connection: &#args) -> Vec<#name>  {
                    use crate::schema::#name_lower::dsl::*;
                    use diesel::prelude::*;

                    let results = #name_lower
                        .load::<#name>(connection)
                        .expect("Error loading #name_ident");

                    results
                }
            }

    }
    .into()
}
