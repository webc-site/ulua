//! `#[derive(FromLua)]` — generates an `impl FromLua for T` that extracts the
//! value from a Lua userdata of type `T` (by `borrow` + `clone`).
//!
//! Faithful port of mlua's `from_lua` derive (`mlua_derive/src/from_lua.rs`):
//! same `Self: 'static + Clone` bound, same match on the userdata value, same
//! `FromLuaConversionError` shape for any other value type. Only the crate path
//! (`::ulua_rt` instead of `::mlua`) and ulua-rt's slightly different
//! `Value`/`Error` field layout differ.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, parse_quote};

pub fn from_lua(input: TokenStream) -> TokenStream {
  let DeriveInput {
    ident,
    mut generics,
    ..
  } = parse_macro_input!(input as DeriveInput);

  let ident_str = ident.to_string();
  generics
    .make_where_clause()
    .predicates
    .push(parse_quote!(Self: 'static + Clone));
  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

  quote! {
      impl #impl_generics ::ulua_rt::FromLua for #ident #ty_generics #where_clause {
          #[inline]
          fn from_lua(value: ::ulua_rt::Value, _: &::ulua_rt::Lua) -> ::ulua_rt::Result<Self> {
              match value {
                  ::ulua_rt::Value::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
                  _ => Err(::ulua_rt::Error::FromLuaConversionError {
                      from: value.type_name(),
                      to: #ident_str.to_string(),
                      message: None,
                  }),
              }
          }
      }
  }
  .into()
}
