mod utils;
pub(crate) use utils::*;

mod inputs;
pub(crate) use inputs::*;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, parse_macro_input};

#[proc_macro]
pub fn broadcast(input: TokenStream) -> TokenStream {
    let PersistanceInput {
        struct_name,
        channel_type,
        tx_alias,
        body,
    } = parse_macro_input!(input as PersistanceInput);

    let fn_name = Ident::new(
        &format!(
            "__{}_broadcast_background_process__",
            struct_name.to_string().to_lowercase()
        ),
        struct_name.span(),
    );

    let const_name = Ident::new(
        &format!("__{}_BROADCAST__", struct_name.to_string().to_uppercase()),
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
    let PersistentInput {
        struct_name,
        storage_type,
        body,
        interval,
    } = parse_macro_input!(input as PersistentInput);

    let fn_name = Ident::new(
        &format!(
            "__{}_persistent_background_process__",
            struct_name.to_string().to_lowercase()
        ),
        struct_name.span(),
    );

    let const_name = Ident::new(
        &format!("__{}_PERSISTENT__", struct_name.to_string().to_uppercase()),
        struct_name.span(),
    );

    let cache_name = Ident::new(
        &format!(
            "__{}_PERSISTENT_CACHE__",
            struct_name.to_string().to_uppercase()
        ),
        struct_name.span(),
    );

    let expanded = quote! {
        async fn #fn_name(tx: ::grapes::tokio::sync::broadcast::Sender<#storage_type>) {
            loop {
                let value = (#body)().await;
                tx.send(value).unwrap();
                let mut cache = #cache_name.write().await;
                *cache = Some(value);
                ::grapes::tokio::time::sleep(#interval).await;
            }
        }

        pub static #const_name: std::sync::LazyLock<::grapes::tokio::sync::broadcast::Sender<#storage_type>> =
            std::sync::LazyLock::new(|| {
                let (tx, _) = ::grapes::tokio::sync::broadcast::channel::<#storage_type>(64);
                ::grapes::RT.spawn(#fn_name(tx.clone()));
                tx
            });

        static #cache_name: ::grapes::tokio::sync::RwLock<Option<#storage_type>> =
            ::grapes::tokio::sync::RwLock::const_new(None);

        pub struct #struct_name;

        impl ::grapes::Service for #struct_name {
            type Message = #storage_type;
        }

        impl ::grapes::Broadcast for #struct_name {
            fn subscribe() -> ::grapes::tokio::sync::broadcast::Receiver<#storage_type> {
                #const_name.subscribe()
            }
        }

        impl Cacheable for #struct_name {
            fn cache() -> ::grapes::tokio::sync::RwLockReadGuard<'static, Self::Message> {
                let cache = #cache_name.blocking_read();
                ::grapes::tokio::sync::RwLockReadGuard::try_map(cache, |opt| opt.as_ref()).unwrap()
            }

            fn cache_copy() -> Self::Message
            where
                Self::Message: Copy,
            {
                #cache_name.blocking_read().unwrap()
            }

            async fn cache_async() -> ::grapes::tokio::sync::RwLockReadGuard<'static, Self::Message> {
                let cache = #cache_name.read().await;
                ::grapes::tokio::sync::RwLockReadGuard::try_map(cache, |opt| opt.as_ref()).unwrap()
            }

            async fn cache_copy_async() -> Self::Message
            where
                Self::Message: Copy,
            {
                #cache_name.read().await.unwrap()
            }
        }
    };

    TokenStream::from(expanded)
}

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
