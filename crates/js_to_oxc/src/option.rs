use crate::JsToOxc;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

impl JsToOxc {
  pub(crate) fn gen_option_with_type<T, M>(
    &self,
    option: &Option<T>,
    r#type: &str,
    map: M,
  ) -> TokenStream
  where
    M: Fn(&T) -> TokenStream,
  {
    let _type = format_ident!("{}", r#type);
    match option {
      Some(value) => {
        let value = map(value);
        quote! {
          Some(#value)
        }
      }
      None => quote! {
        NONE
      },
    }
  }

  pub(crate) fn gen_option<T, M, R>(&self, option: &Option<T>, map: M) -> TokenStream
  where
    M: Fn(&T) -> R,
    R: ToTokens,
  {
    match option {
      Some(value) => {
        let value = map(value);
        quote! {
          Some(#value)
        }
      }
      None => quote! {
        None
      },
    }
  }
}
