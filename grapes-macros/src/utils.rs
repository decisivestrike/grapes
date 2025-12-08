use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{GenericArgument, PathArguments, Type};

/// Allows to get `T` from `SomeType<T>`
pub fn extract_generic(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        let segment = type_path.path.segments.last()?;

        if let PathArguments::AngleBracketed(ref args) = segment.arguments
            && let Some(GenericArgument::Type(inner_ty)) = args.args.first()
        {
            return Some(inner_ty);
        }
    }

    None
}

pub fn error<T>(span: T, message: &str) -> TokenStream
where
    T: ToTokens,
{
    syn::Error::new_spanned(span, message)
        .to_compile_error()
        .into()
}
