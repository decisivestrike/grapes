use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, FnArg, GenericArgument, Ident, ItemFn, PathArguments, Type,
    parse_macro_input,
};

#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let initial_fn: TokenStream2 = item.clone().into();

    let item_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &item_fn.sig.ident;

    let struct_name = parse_macro_input!(attr as Ident);
    let const_name = Ident::new(&fn_name.to_string().to_uppercase(), Span::call_site());
    let mut t = None;

    for input in &item_fn.sig.inputs {
        if let FnArg::Typed(pat_type) = input
            && let Type::Path(type_path) = &*pat_type.ty
            && let Some(segment) = type_path.path.segments.last()
            && segment.ident == "Sender"
            && let PathArguments::AngleBracketed(angle_bracketed) = &segment.arguments
        {
            for arg in &angle_bracketed.args {
                if let GenericArgument::Type(inner_ty) = arg {
                    t = Some(inner_ty);
                }
            }
        }
    }

    let t = t.unwrap();

    let expanded = quote! {
        #initial_fn

        pub static #const_name: std::sync::LazyLock<tokio::sync::broadcast::Sender<#t>> = std::sync::LazyLock::new(|| {
            let (tx, _) = broadcast::channel(64);
            RT.spawn(#fn_name(tx.clone()));
            tx
        });

        pub struct #struct_name;

        impl Service<#t> for #struct_name {
            fn subscribe() -> tokio::sync::broadcast::Receiver<#t> {
                #const_name.subscribe()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ComponentBase, attributes(root))]
pub fn special_getter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let mut getter = None;

    if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(named_fields) = data_struct.fields {
            for field in named_fields.named.iter() {
                for attr in &field.attrs {
                    if attr.path().is_ident("root") {
                        let field_name = &field.ident;
                        let field_type = &field.ty;

                        getter = Some(quote! {
                            impl ComponentBase for #struct_name {
                                type Root = #field_type;

                                fn root(&self) -> Self::Root {
                                    self.#field_name.clone()
                                }

                                fn as_widget_ref(&self) -> &gtk::Widget {
                                    &self.#field_name
                                }
                            }
                        });
                    }
                }
            }
        }
    }
    getter.unwrap_or_else(|| quote! {}).into()
}
