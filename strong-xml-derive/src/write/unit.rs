use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

pub fn write(tag: &LitStr, ele_name: TokenStream) -> TokenStream {
    quote! {
        log::debug!(concat!("[", stringify!(#ele_name), "] Started writing."));

        writer.write_element_start(#tag)?;
        writer.write_element_end_empty()?;

        log::debug!(concat!("[", stringify!(#ele_name), "] Finished writing."));
    }
}