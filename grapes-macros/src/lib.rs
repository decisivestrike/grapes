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

fn get_type_name(ty: &syn::Type) -> Option<String> {
    if let Type::Path(typepath) = ty {
        if let Some(last_segment) = typepath.path.segments.last() {
            return Some(last_segment.ident.to_string());
        }
    }

    None
}

#[proc_macro_derive(GtkCompatible, attributes(root))]
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

                        let ffi_struct_name = Ident::new(
                            &format!("Gtk{}", get_type_name(&field_type).unwrap()),
                            Span::call_site(),
                        );

                        getter = Some(quote! {
                        impl GtkCompatible for #struct_name {
                            fn as_widget_ref(&self) -> &::grapes::gtk::Widget {
                                self.#field_name.upcast_ref()
                            }
                        }

                        impl std::convert::Into<::grapes::gtk::Widget> for #struct_name {
                            fn into(self) -> ::grapes::gtk::Widget {
                                self.#field_name.clone().into()
                            }
                        }

                        impl ::std::convert::AsRef<::grapes::gtk::Widget> for #struct_name {
                            fn as_ref(&self) -> &::grapes::gtk::Widget {
                                self.as_widget_ref()
                            }
                        }

                        impl ::std::borrow::Borrow<::grapes::gtk::Widget> for #struct_name {
                            fn borrow(&self) -> &::grapes::gtk::Widget {
                                self.as_widget_ref()
                            }
                        }

                        impl ::grapes::glib::translate::IntoGlibPtr<*mut ::grapes::gtk::ffi::#ffi_struct_name> for #struct_name {
                            fn into_glib_ptr(self) -> *mut ::grapes::gtk::ffi::#ffi_struct_name {
                                self.#field_name.into_glib_ptr()
                            }
                        }

                        impl<'a> ::grapes::glib::translate::ToGlibPtr<'a, *mut ::grapes::gtk::ffi::#ffi_struct_name> for #struct_name {
                            type Storage = &'a #struct_name;

                            fn to_glib_none(
                                &'a self,
                            ) -> ::grapes::gtk::glib::translate::Stash<'a, *mut ::grapes::gtk::ffi::#ffi_struct_name, Self> {
                                use grapes::glib::translate::IntoGlibPtr;
                                let ptr = self.#field_name.clone().into_glib_ptr();
                                ::grapes::gtk::glib::translate::Stash(ptr, self)
                            }
                        }

                        impl ::grapes::glib::value::ToValueOptional for #struct_name {
                            fn to_value_optional(maybe_component: Option<&Self>) -> ::grapes::glib::Value {
                                match maybe_component {
                                    Some(component) => component.to_value(),
                                    None => ::grapes::glib::Value::from(None::<&#field_type>),
                                }
                            }
                        }

                        impl ::grapes::glib::value::ToValue for #struct_name {
                            fn to_value(&self) -> ::grapes::glib::Value {
                                self.#field_name.to_value()
                            }

                            fn value_type(&self) -> ::grapes::glib::Type {
                                self.#field_name.value_type()
                            }
                        }

                        unsafe impl<'a> ::grapes::glib::value::FromValue<'a> for #struct_name {
                            type Checker = <#field_type as ::grapes::glib::value::FromValue<'a>>::Checker;

                            unsafe fn from_value(_: &'a ::grapes::glib::Value) -> Self {
                                unimplemented!()
                            }
                        }

                        impl ::grapes::glib::value::ValueType for #struct_name {
                            type Type = <#field_type as ValueType>::Type;
                        }

                        impl ::grapes::glib::types::StaticType for #struct_name {
                            fn static_type() -> ::grapes::glib::Type {
                                #field_type::static_type()
                            }
                        }

                        impl ::grapes::glib::translate::UnsafeFrom<::grapes::glib::object::ObjectRef> for #struct_name {
                            unsafe fn unsafe_from(_: ::grapes::glib::object::ObjectRef) -> Self {
                                unimplemented!()
                            }
                        }

                        impl ::std::convert::From<#struct_name> for ::grapes::glib::object::ObjectRef {
                            fn from(value: #struct_name) -> Self {
                                value
                                    .#field_name
                                    .upcast::<::grapes::glib::object::Object>()
                                    .into()
                            }
                        }

                        unsafe impl ::grapes::glib::object::ObjectType for #struct_name {
                            type GlibType = <#field_type as ObjectType>::GlibType;

                            type GlibClassType = <#field_type as ObjectType>::GlibClassType;

                            fn as_object_ref(&self) -> &::grapes::glib::object::ObjectRef {
                                self.#field_name.as_object_ref()
                            }

                            fn as_ptr(&self) -> *mut Self::GlibType {
                                self.#field_name.as_ptr()
                            }

                            unsafe fn from_glib_ptr_borrow(_: &*mut Self::GlibType) -> &Self {
                                unimplemented!()
                            }
                        }

                        unsafe impl ::grapes::glib::object::IsA<::grapes::gtk::Widget> for #struct_name {}
                        });
                    }
                }
            }
        }
    }
    getter.unwrap_or_else(|| quote! {}).into()
}
