use syn::{
    Expr, Ident, Token, Type,
    parse::{Parse, ParseStream},
    token::Comma,
};

pub(crate) struct PersistanceInput {
    pub struct_name: Ident,
    pub channel_type: Type,
    pub tx_alias: Ident,
    pub body: Expr,
}

impl Parse for PersistanceInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_name = input.parse()?;
        input.parse::<Token![->]>()?;

        let channel_type = input.parse()?;
        input.parse::<Comma>()?;
        input.parse::<Token![async]>()?;
        input.parse::<Token![|]>()?;

        let tx_alias = input.parse()?;
        input.parse::<Token![|]>()?;

        let body = input.parse()?;

        Ok(PersistanceInput {
            struct_name,
            channel_type,
            tx_alias,
            body,
        })
    }
}

pub(crate) struct PersistentInput {
    pub struct_name: Ident,
    pub storage_type: Type,
    pub body: Expr,
    pub interval: Expr,
}

impl Parse for PersistentInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_name = input.parse()?;
        input.parse::<Token![->]>()?;

        let storage_type = input.parse()?;
        input.parse::<Comma>()?;

        let body = input.parse()?;
        input.parse::<Comma>()?;

        let interval = input.parse()?;

        Ok(PersistentInput {
            struct_name,
            storage_type,

            body,
            interval,
        })
    }
}
