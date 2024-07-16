extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(IntoProtobuf)]
pub fn into_protobuf(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let gen = impl_into_protobuf(&ast);

    gen
}

fn impl_into_protobuf(ast: &syn::DeriveInput) -> TokenStream {
    let rust_type_name = &ast.ident;
    let protobuf_type_name = format_ident!("Proto{}", rust_type_name);
    let fields = match &ast.data {
        syn::Data::Struct(s) => &s.fields,
        syn::Data::Enum(_) => panic!("into_protobuf not implemented on Enums"),
        syn::Data::Union(_) => panic!("into_protobuf not implemented on Unions"),
    };
    let fieldnames: Vec<Ident> = fields.iter().map(|x| x.ident.clone().unwrap()).collect();
    let prefixed_fields: Vec<proc_macro2::TokenStream> =
        fieldnames.iter().map(|x| quote!(value.#x)).collect();
    let maps: Vec<proc_macro2::TokenStream> = (fieldnames.iter().zip(prefixed_fields.iter()))
        .map(|(n, pf)| create_map(n.clone(), pf))
        .collect();

    let fromdef = create_from_definition(&maps, &rust_type_name, &protobuf_type_name);

    quote! {
        #fromdef
        impl IntoProtobuf for #rust_type_name {
            fn internal_print_into_protobuf_debug_info() {
                println!("Implementing {} for {}", stringify!(#protobuf_type_name), stringify!(#rust_type_name));
                println!("Field names: {}", stringify!(#(#fieldnames),*));
                println!("Prefixed Field names: {}", stringify!(#(#prefixed_fields),*));
                println!("Prefixed Field names: {}", stringify!(#(#maps),*));
                println!("From Def: {}", stringify!(#fromdef))
            }
        }
    }
    .into()
}

fn create_map(
    fieldname: Ident,
    prefixed_field: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        #fieldname : #prefixed_field
    }
}

fn create_from_definition(
    maps: &Vec<proc_macro2::TokenStream>,
    rust_type_name: &Ident,
    protobuf_type_name: &Ident,
) -> proc_macro2::TokenStream {
    quote! {
        impl From<#rust_type_name> for #protobuf_type_name {
            fn from(value: #rust_type_name) -> #protobuf_type_name {
                #protobuf_type_name {
                    #(#maps),*
                }
            }
        }

        impl From<#protobuf_type_name> for #rust_type_name {
            fn from(value: #protobuf_type_name) -> #rust_type_name {
                #rust_type_name {
                    #(#maps),*
                }
            }
        }
    }
}
