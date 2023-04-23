use syn::{parse::Parse, Expr, Token, Type};

pub(crate) struct GetFunky {
    pub method_name: syn::Ident,
    pub mut_get: bool,
    pub from: Expr,
    pub ret_type: Type,
}

impl Parse for GetFunky {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let method_name = input.parse::<syn::Ident>()?;
        input.parse::<Token!(::)>()?;
        let mut_get = if input.peek(Token![mut]) {
            input.parse::<Token![mut]>()?;
            true
        } else {
            false
        };

        let from = input.parse::<Expr>()?;

        input.parse::<Token!(=>)>()?;

        let ret_type = input.parse::<Type>()?;
        Ok(Self {
            method_name,
            mut_get,
            from,
            ret_type,
        })
    }
}
