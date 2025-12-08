mod utils;
pub(crate) use utils::*;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Data, DeriveInput, Expr, Fields, Ident, Token, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Comma,
};

struct BroadcastInput {
    struct_name: Ident,
    _arrow_token: Token![->],
    channel_type: Type,
    _comma_token: Comma,
    _async: Token![async],
    _bar1: Token![|],
    tx_alias: Ident,
    _bar2: Token![|],
    body: Expr,
}

impl Parse for BroadcastInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(BroadcastInput {
            struct_name: input.parse()?,
            _arrow_token: input.parse()?,
            channel_type: input.parse()?,
            _comma_token: input.parse()?,
            _async: input.parse()?,
            _bar1: input.parse()?,
            tx_alias: input.parse()?,
            _bar2: input.parse()?,
            body: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn broadcast(input: TokenStream) -> TokenStream {
    let BroadcastInput {
        struct_name,
        channel_type,
        tx_alias,
        body,
        ..
    } = parse_macro_input!(input as BroadcastInput);

    let fn_name = Ident::new(
        &format!(
            "__{}_background_process__",
            struct_name.to_string().to_lowercase()
        ),
        struct_name.span(),
    );

    let const_name = Ident::new(
        &format!("__{}__", struct_name.to_string().to_uppercase()),
        struct_name.span(),
    );

    let expanded = quote! {
        async fn #fn_name(#tx_alias: ::grapes::tokio::sync::broadcast::Sender<#channel_type>) #body

        pub static #const_name: std::sync::LazyLock<::grapes::tokio::sync::broadcast::Sender<#channel_type>> =
            std::sync::LazyLock::new(|| {
                let (tx, _) = ::grapes::tokio::sync::broadcast::channel::<#channel_type>(64);
                ::grapes::RT.spawn(#fn_name(tx.clone()));
                tx
            });

        pub struct #struct_name;

        impl ::grapes::Service for #struct_name {
            type Message = #channel_type;
        }

        impl ::grapes::Broadcast for #struct_name {
            fn subscribe() -> ::grapes::tokio::sync::broadcast::Receiver<#channel_type> {
                #const_name.subscribe()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn persistent(input: TokenStream) -> TokenStream {
    let BroadcastInput {
        struct_name,
        channel_type,
        tx_alias,
        body,
        ..
    } = parse_macro_input!(input as BroadcastInput);

    let fn_name = Ident::new(
        &format!(
            "__{}_background_process__",
            struct_name.to_string().to_lowercase()
        ),
        struct_name.span(),
    );

    let const_name = Ident::new(
        &format!("__{}__", struct_name.to_string().to_uppercase()),
        struct_name.span(),
    );

    let expanded = quote! {
        async fn #fn_name(#tx_alias: ::grapes::tokio::sync::broadcast::Sender<#channel_type>) #body

        pub static #const_name: std::sync::LazyLock<::grapes::tokio::sync::broadcast::Sender<#channel_type>> =
            std::sync::LazyLock::new(|| {
                let (tx, _) = ::grapes::tokio::sync::broadcast::channel::<#channel_type>(64);
                ::grapes::RT.spawn(#fn_name(tx.clone()));
                tx
            });

        pub struct #struct_name {
            cache: #channel_type,
        }

        impl ::grapes::Service for #struct_name {
            type Message = #channel_type;
        }

        impl ::grapes::Cacheable for #struct_name {
            fn cache(&self) -> &#channel_type {
                &self.cache
            }
        }
    };

    TokenStream::from(expanded)
}

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
                }

                if attr.path().is_ident("state") {
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
