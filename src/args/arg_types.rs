use syn::{parenthesized, parse::Parse};

use super::{funky::GetFunky, kw};

pub(crate) enum GetArgs {
    Copy,
    Clone,
    ImRef(syn::Type),
    MutRef(syn::Type),
    Funky(GetFunky),
}

impl Parse for GetArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let look = input.lookahead1();
        let inner;
        if look.peek(kw::copy) {
            input.parse::<kw::copy>()?;
            Ok(Self::Copy)
        } else if look.peek(kw::clone) {
            input.parse::<kw::clone>()?;
            Ok(Self::Clone)
        } else if look.peek(kw::im_ref) {
            input.parse::<kw::im_ref>()?;
            parenthesized!(inner in input);
            let ret_type = inner.parse::<syn::Type>()?;
            Ok(Self::ImRef(ret_type))
        } else if look.peek(kw::mut_ref) {
            input.parse::<kw::mut_ref>()?;
            parenthesized!(inner in input);
            let ret_type = inner.parse::<syn::Type>()?;
            Ok(Self::MutRef(ret_type))
        } else if look.peek(kw::funky) {
            input.parse::<kw::funky>()?;
            parenthesized!(inner in input);
            let funky = inner.parse::<GetFunky>()?;
            Ok(Self::Funky(funky))
        } else {
            Err(look.error())
        }
    }
}
