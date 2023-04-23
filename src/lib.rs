mod args;

use args::arg_types::GetArgs;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse::ParseStream, parse_macro_input, punctuated::Punctuated, spanned::Spanned, Fields,
    ItemStruct, Meta, Token, Type, Visibility,
};

#[proc_macro_derive(GetSet, attributes(get, set))]
pub fn get_set(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);

    let struct_name = item_struct.ident;
    let fields = item_struct.fields;

    let methods = process_fields(&fields);
    if methods.is_ok() {
        let methods = methods.unwrap();
        quote!(
            impl #struct_name {
                #(#methods)*
            }
        )
        .into()
    } else {
        let err = methods.unwrap_err();
        err.into_compile_error().into()
    }
}

fn validate_get_args(attrs: &Punctuated<GetArgs, Token![,]>) -> Option<&'static str> {
    let (copy, clone, im_ref, mut_ref) = attrs.iter().fold((0, 0, 0, 0), |mut curr, arg| {
        match arg {
            GetArgs::Copy => curr.0 += 1,
            GetArgs::Clone => curr.1 += 1,
            GetArgs::ImRef(_) => curr.2 += 1,
            GetArgs::MutRef(_) => curr.3 += 1,
            _ => {}
        }
        curr
    });

    match (copy, clone, im_ref, mut_ref) {
        (1, 1, _, _) => Some("`copy` and `clone` can not be used simultaneously"),
        (0 | 1, 0 | 1, 0 | 1, 0 | 1) => None, 
        _ => Some("only `funky` can be present more than once, rest all attribute helpers can be present at most once")
    }
}

fn transform_get(
    attrs: &Punctuated<GetArgs, Token![,]>,
    vis: &Visibility,
    field_name: &Ident,
    ty: &Type,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    if attrs.is_empty() {
        return Err(syn::Error::new(
            field_name.span(),
            "must specify a way to get the value from this field: 
            `#[get(copy)]`, `#[get(clone)]`, `#[get(im_ref(..))]`, `#[get(mut_ref(..))]`, `#[get(funky(..))]`",
        ));
    }

    Ok(attrs
        .iter()
        .map(|arg| match arg {
            GetArgs::Copy => {
                let method_name = Ident::new(&format!("get_{}", field_name), field_name.span());
                quote!(
                #vis fn #method_name(&self) -> #ty {
                   self.#field_name
                   }
                )
            }
            GetArgs::Clone => {
                let method_name = Ident::new(&format!("get_{}", field_name), field_name.span());
                quote!(
                    #vis fn #method_name(&self) -> #ty {
                        self.#field_name.clone()
                    }
                )
            }
            GetArgs::ImRef(ret_ty) => {
                let method_name = Ident::new(&format!("get_{}_ref", field_name), field_name.span());
                quote!(
                    #vis fn #method_name(&self) -> #ret_ty {
                        self.#field_name.as_ref()
                    }
                )
            }
            GetArgs::MutRef(ret_ty) => {
                let method_name =
                    Ident::new(&format!("get_{}_ref_mut", field_name), field_name.span());
                quote!(
                    #vis fn #method_name(&self) -> #ret_ty {
                        self.#field_name.as_mut()
                    }
                )
            }
            GetArgs::Funky(funky) => {
                let method_name =
                    Ident::new(&format!("get_{}", funky.method_name), field_name.span());
                let ret_ty = &funky.ret_type;
                let from = &funky.from;

                if !funky.mut_get {
                    quote!(
                    #vis fn #method_name(&self) -> #ret_ty {
                        let #field_name = &self.#field_name;
                        let __res = { #from };
                        __res
                    })
                } else {
                    quote!(
                    #vis fn #method_name(&mut self) -> #ret_ty {
                        let #field_name = &mut self.#field_name;
                        let __res = { #from };
                        __res
                    })
                }
            }
        })
        .collect())
}

fn process_fields(fields: &Fields) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    fields
        .iter()
        .try_fold(vec![], |mut final_tokens, f| -> syn::Result<_> {
            let vis = &f.vis;
            let field_name = f.ident.as_ref().unwrap();
            let ty = &f.ty;

            let mut token_vec = vec![];

            if f.attrs.iter().filter(|a| a.path.is_ident("get")).count() > 1
                || f.attrs.iter().filter(|a| a.path.is_ident("set")).count() > 1
            {
                return Err(syn::Error::new(
                    f.span(),
                    "`#[get]` and `#[set]` are allowed atmost once",
                ));
            }

            if let Some(attr) = f.attrs.iter().find(|a| a.path.is_ident("get")) {
                let get_args = attr.parse_args_with(|input: ParseStream| -> syn::Result<_> {
                    Punctuated::<GetArgs, Token![,]>::parse_terminated(input)
                })?;

                if let Some(err) = validate_get_args(&get_args) {
                    return Err(syn::Error::new(f.span(), err));
                }

                token_vec.extend(transform_get(&get_args, vis, field_name, ty)?);
            }

            if let Some(attr) = f.attrs.iter().find(|a| a.path.is_ident("set")) {
                match attr.parse_meta()? {
                    Meta::Path(_) => {}
                    _ => return Err(syn::Error::new(attr.span(), "only `#[set]` supported")),
                }
                let method_name = Ident::new(&format!("set_{}", field_name), attr.span());
                token_vec.push(quote!(
                    #vis fn #method_name(&mut self, value: #ty) {
                        self.#field_name = value;
                    }
                ))
            }

            final_tokens.extend(token_vec);
            Ok(final_tokens)
        })
}
