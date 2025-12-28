mod utils;
pub(crate) use utils::*;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// Generates code that will have to be written anyway
#[proc_macro_derive(GtkCompatible, attributes(root, state))]
pub fn gtk_compatible(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let mut maybe_root_ts: Option<TokenStream2> = None;
    let mut maybe_state_ts: Option<TokenStream2> = None;

    if let Data::Struct(data_struct) = input.data
        && let Fields::Named(named_fields) = data_struct.fields
    {
        for field in named_fields.named.iter() {
            for attr in &field.attrs {
                if attr.path().is_ident("root") {
                    let field_name = &field.ident;

                    if maybe_root_ts.is_some() {
                        return error(
                            struct_name,
                            "Only one field can have the #[root] attribute.",
                        );
                    }

                    let expanded = quote! {
                        impl GtkCompatible for #struct_name {
                            fn as_widget_ref(&self) -> &::grapes::gtk::Widget {
                                use ::grapes::gtk::prelude::Cast;
                                self.#field_name.upcast_ref()
                            }
                        }

                        impl AsRef<::grapes::gtk::Widget> for #struct_name {
                            fn as_ref(&self) -> &::grapes::gtk::Widget {
                                self.as_widget_ref()
                            }
                        }
                    };

                    maybe_root_ts = Some(expanded);
                } else if attr.path().is_ident("state") {
                    let field_name = &field.ident;
                    let field_type = &field.ty;

                    if let Some(_) = maybe_state_ts {
                        return error(
                            attr.path(),
                            "Using the #[state] attribute twice",
                        );
                    }

                    let generic_type = match extract_generic(field_type) {
                        Some(t) => t,
                        None => {
                            return error(
                                struct_name,
                                "Can't extract T from State<T>",
                            );
                        }
                    };

                    let expanded = quote! {
                        impl ::grapes::Updateable for #struct_name {
                            type Message = #generic_type;

                            fn update(&self, value: #generic_type) {
                                self.#field_name.set(value);
                            }
                        }
                    };

                    maybe_state_ts = Some(expanded);
                }
            }
        }
    }

    match (maybe_root_ts, maybe_state_ts) {
        (None, _) => error(
            struct_name,
            "One of the fields must have the #[root] attribute.",
        ),
        (Some(root_ts), None) => root_ts.into(),
        (Some(root_ts), Some(state_ts)) => {
            let mut ts = TokenStream2::new();
            ts.extend(root_ts);
            ts.extend(state_ts);
            ts.into()
        }
    }
}
