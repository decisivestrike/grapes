use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Expr, Fields, Ident, Token, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Comma,
};

struct ServiceInput {
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

impl Parse for ServiceInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ServiceInput {
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
pub fn service(input: TokenStream) -> TokenStream {
    let ServiceInput {
        struct_name,
        channel_type,
        tx_alias,
        body,
        ..
    } = parse_macro_input!(input as ServiceInput);

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

        impl ::grapes::Service<#channel_type> for #struct_name {
            fn subscribe() -> ::grapes::tokio::sync::broadcast::Receiver<#channel_type> {
                #const_name.subscribe()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(GtkCompatible, attributes(root))]
pub fn gtk_compatible(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let mut maybe_ts = None;

    if let Data::Struct(data_struct) = input.data
        && let Fields::Named(named_fields) = data_struct.fields
    {
        for field in named_fields.named.iter() {
            for attr in &field.attrs {
                if attr.path().is_ident("root") {
                    let field_name = &field.ident;

                    if maybe_ts.is_some() {
                        return syn::Error::new_spanned(
                            struct_name,
                            "Only one field can have the #[root] attribute.",
                        )
                        .to_compile_error()
                        .into();
                    }

                    maybe_ts = Some(quote! {
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
                    });
                }
            }
        }
    }

    match maybe_ts {
        Some(ts) => ts.into(),
        None => syn::Error::new_spanned(
            struct_name,
            "One of the fields must have the #[root] attribute.",
        )
        .to_compile_error()
        .into(),
    }
}
