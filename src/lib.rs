//! # Same Enum
//!
//! `same_enum` generates `From` trait implementations for enums with
//! the same unit variants.
//!
//! # Get Started
//!
//! Place a `#[from(path::to::TargetEnum)]` or `#[to(path::to::TargetEnum)]`
//! attribute above the source enum definition to generate `From` trait
//! implementations.  

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Generates an implementation of the From trait to convert from a target enum to
/// the annotated enum.
///
/// # Examples
///
/// ```
/// use same_enum::from;
/// #[from(TargetEnum)]
/// #[derive(PartialEq, Debug)]
/// enum SourceEnum {
///     One,
///     Two,
/// }
/// #[derive(PartialEq, Debug)]
/// enum TargetEnum {
///     One,
///     Two,
/// }
///
/// let target_one = TargetEnum::One;
/// let source_one: SourceEnum = target_one.into(); // from implementation gives us into() as well
///
/// assert_eq!(source_one, SourceEnum::One);
/// ```
///
/// # Panics
///
/// `same_enum` panics if the annotated_item is not an enum with Unit variants.
///
/// An example of an enum **with** Unit variants would be:
/// ```
/// enum BinaryNumbers {
///     Zero,
///     One,
/// }
/// ```
#[proc_macro_attribute]
pub fn from(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let target_enum_path = parse_macro_input!(input as syn::Path);
    let input_enum = parse_macro_input!(annotated_item as DeriveInput);

    let input_enum_name = input_enum.clone().ident;

    let data = match input_enum.clone().data {
        Data::Enum(data) => data,
        _ => panic!("convert_enum can only be applied to enums"),
    };

    let mut to_variants = Vec::new();

    for variant in data.variants {
        let variant_name = variant.ident;
        match variant.fields {
            Fields::Unit => {
                to_variants.push(quote! {
                    #target_enum_path::#variant_name => #input_enum_name::#variant_name,
                });
            }
            _ => panic!("same_enum does not support unnamed or named fields"),
        }
    }

    let expanded = quote! {
        impl From<#target_enum_path> for #input_enum_name {
            fn from(value: #target_enum_path) -> Self {
                match value {
                    #(#to_variants)*
                }
            }
        }
    };

    let output = quote! {
        #input_enum
        #expanded
    };

    TokenStream::from(output)
}

/// Generates an implementation of the From trait to convert from the annotated enum to a target enum.
///
/// # Examples
///
/// ```
/// use same_enum::to;
/// #[to(TargetEnum)]
/// #[derive(PartialEq, Debug)]
/// enum SourceEnum {
///     One,
///     Two,
/// }
/// #[derive(PartialEq, Debug)]
/// enum TargetEnum {
///     One,
///     Two,
/// }
///
/// let source_one = SourceEnum::One;
/// let target_one = TargetEnum::from(source_one);
///
/// assert_eq!(target_one, TargetEnum::One);
/// ```
/// # Panics
///
/// `same_enum` panics if the annotated_item is not an enum with Unit variants.
///
/// An example of an enum **with** Unit variants would be:
/// ```
/// enum BinaryNumbers {
///     Zero,
///     One,
/// }
/// ```
#[proc_macro_attribute]
pub fn to(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let target_enum_path = parse_macro_input!(input as syn::Path);
    let input_enum = parse_macro_input!(annotated_item as DeriveInput);

    let input_enum_name = input_enum.clone().ident;

    let data = match input_enum.clone().data {
        Data::Enum(data) => data,
        _ => panic!("convert_enum can only be applied to enums"),
    };

    let mut from_variants = Vec::new();

    for variant in data.variants {
        let variant_name = variant.ident;
        match variant.fields {
            Fields::Unit => {
                from_variants.push(quote! {
                    #input_enum_name::#variant_name => #target_enum_path::#variant_name,
                });
            }
            _ => panic!("same_enum does not support unnamed or named fields"),
        }
    }

    let expanded = quote! {
        impl From<#input_enum_name> for #target_enum_path {
            fn from(value: #input_enum_name) -> Self {
                match value {
                    #(#from_variants)*
                }
            }
        }
    };

    let output = quote! {
        #input_enum
        #expanded
    };

    TokenStream::from(output)
}
