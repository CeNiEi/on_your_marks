pub mod arg_types;
pub mod funky;

mod kw {
    syn::custom_keyword!(copy);
    syn::custom_keyword!(clone);
    syn::custom_keyword!(mut_ref);
    syn::custom_keyword!(im_ref);
    syn::custom_keyword!(funky);
}
