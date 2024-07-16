use proc_macro::{self, TokenStream}
use quote::quote;
use syn::spanned::Spanned;
use syn::parse::{ Parse, Result };
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn not_the_bees(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the token stream into a syntax tree.
    // This syntax tre is specifically an Item.
    // An item is a syntax item that can appear at the module level. This includes
    // function and struct definitions.
    let item: syn::Item = syn::parse(input).expect("Failed to parse input");

    match item {
        // We only care about structs here.
        Item::Struct(ref struct_item) => {
            if has_bees(struct_item) {
                light_it_up(struct_item);
            }
        },
        _ => {
        }
    }

    // We can convert the syntax tree back into tokens so we can return them.
    let output = quote!{#item};
    output.into()
}

/// Determines if the struct has a field named Bees.
fn has_bees(target: &syn::ItemStruct) -> bool {
    unimplemented!()
}

/// Generate fun compiler errors.
fn light_it_up(target: &syn::ItemStruct) {
    unimplemented!()
}
