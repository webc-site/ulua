//! `#[derive(UserData)]` — generates an `impl UserData for T` that exposes the
//! struct's fields to Lua.
//!
//! ## Relation to mlua
//!
//! mlua's `#[derive(UserData)]` (`mlua_derive/src/userdata/mod.rs`) handles the
//! struct's **named fields** directly — registering an `add_field_method_get`
//! (and `add_field_method_set` unless the field is read-only) per field — and
//! defers *methods* to a separate `#[userdata_impl]` attribute macro plus an
//! `inventory`-based registry. ulua-rt has neither `inventory` nor a
//! `UserDataRegistry`, and its `UserData` trait uses the `add_fields` /
//! `add_methods` shape (mirroring mlua 0.9). So this derive emits a direct
//! `impl UserData for T` whose `add_fields` registers the field getters/setters
//! — faithful to the **field** behaviour of mlua's derive, with the same
//! `#[lua(...)]` field attributes (`skip`, `get`, `set`, `name = "..."`).
//!
//! The default (no `#[lua(get/set)]`) is get + set, exactly like mlua. A field
//! getter clones the field (`Ok(this.field.clone())`); a setter moves the
//! incoming value into the field. Both require the field type to be `Clone`
//! (getter) / `FromLua` (setter) / `IntoLua` (getter) — the same requirements
//! mlua's generated getters/setters impose.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
  Attribute, Data, DeriveInput, Error, Fields, FieldsNamed, LitStr, Meta, Result, ext::IdentExt,
  spanned::Spanned,
};

use crate::attr::LuaAttr;

/// Parse and merge all `#[lua(...)]` attributes on a field into one [`LuaAttr`].
/// Mirrors mlua's `parse_field_lua_attr`.
fn parse_field_lua_attr(attrs: &[Attribute]) -> Result<LuaAttr> {
  let mut lua_attr = LuaAttr::default();
  for attr in attrs.iter().filter(|attr| attr.path().is_ident("lua")) {
    // 诊断 span 只记最后一个 `#[lua(...)]`（两种形态都需要），故提到分支外统一赋值。
    lua_attr.span = Some(attr.span());
    match &attr.meta {
      // `#[lua(...)]`
      Meta::List(_) => {
        attr.parse_nested_meta(|meta| lua_attr.parse_inner(meta))?;
      }
      // bare `#[lua]` — equivalent to default get + set.
      Meta::Path(_) => {}
      Meta::NameValue(_) => {
        return Err(Error::new_spanned(
          attr,
          "`#[lua = \"...\"]` is not supported: use `#[lua(name = \"...\")]`",
        ));
      }
    }
  }
  Ok(lua_attr)
}

/// derive 入口：内部 [`derive`] 的诊断以 `syn::Error` 经 `?` 上抛，此处单点转成
/// `to_compile_error`，替代此前每个早退分支各自手写 `.to_compile_error().into()`。
pub fn userdata_type(item: TokenStream) -> TokenStream {
  match derive(item) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

/// 校验输入并生成 `impl UserData`。
fn derive(item: TokenStream) -> Result<proc_macro2::TokenStream> {
  let input: DeriveInput = syn::parse(item)?;
  let type_name = &input.ident;

  // Generic type parameters are not supported (mlua rejects them too — the
  // registry needs a concrete type). Lifetimes/const generics share the same
  // limitation here.
  if !input.generics.params.is_empty() {
    return Err(Error::new_spanned(
      &input.generics,
      "`#[derive(UserData)]` does not support generic parameters; \
             wrap the generic type in a concrete newtype instead",
    ));
  }

  let named_fields: Option<&FieldsNamed> = match &input.data {
    Data::Struct(data) => match &data.fields {
      Fields::Named(fields) => Some(fields),
      // Tuple structs / unit structs / enums expose no fields. They still get
      // a valid (empty-field) `UserData` impl — the Rust type registers as a
      // userdata with no Lua-visible fields. (mlua instead rejects these; the
      // derive here stays permissive, the impl is simply fieldless.)
      Fields::Unnamed(_) | Fields::Unit => None,
    },
    Data::Enum(_) => None,
    Data::Union(_) => {
      return Err(Error::new_spanned(
        &input,
        "`#[derive(UserData)]` cannot be applied to unions",
      ));
    }
  };

  let mut field_registrations = Vec::new();
  if let Some(fields) = named_fields {
    for field in &fields.named {
      let field_name = field.ident.as_ref().unwrap();
      let lua_attr = parse_field_lua_attr(&field.attrs)?;
      // `skip` is meaningless combined with `get`/`set`/`name`.
      if lua_attr.skip && (lua_attr.get || lua_attr.set || lua_attr.name.is_some()) {
        return Err(Error::new(
          lua_attr.span(),
          "`skip` cannot be combined with `get`, `set`, or `name`",
        ));
      }
      if lua_attr.skip {
        continue;
      }

      let lua_name = lua_attr
        .name
        .unwrap_or_else(|| LitStr::new(&field_name.unraw().to_string(), field_name.span()));

      // Default (neither `get` nor `set` given) is get + set, like mlua.
      let has_get = lua_attr.get || !lua_attr.set;
      let has_set = lua_attr.set || !lua_attr.get;

      if has_get {
        field_registrations.push(quote! {
            fields.add_field_method_get(
                #lua_name,
                |_lua, this| ::core::result::Result::Ok(this.#field_name.clone()),
            );
        });
      }
      if has_set {
        field_registrations.push(quote! {
            fields.add_field_method_set(#lua_name, |_lua, this, val| {
                this.#field_name = val;
                ::core::result::Result::Ok(())
            });
        });
      }
    }
  }

  Ok(quote! {
      impl ::ulua_rt::UserData for #type_name {
          fn add_fields<__UluaUDF: ::ulua_rt::UserDataFields<Self>>(fields: &mut __UluaUDF) {
              #(#field_registrations)*
          }
      }
  })
}

// §8 留证：本 crate 是 proc-macro crate，唯一公开面为 derive 宏本身，tests/ 集成测试
// 无法把 crate 当普通库链接（proc-macro 不能 `use` 其内部项）；被测
// `parse_field_lua_attr` 为私有属性解析函数，其字面量字节保留/多属性合并语义
// 在展开失败的编译错误里不可观察，只能就近做单元断言，保留 src。
#[cfg(test)]
mod tests {
  use syn::{Field, parse_quote};

  use super::parse_field_lua_attr;

  #[test]
  fn field_name_preserves_literal_bytes() {
    let field: Field = parse_quote! {
      #[lua(get, name = "field\0名")]
      value: u32
    };
    let attr = parse_field_lua_attr(&field.attrs).unwrap();
    assert_eq!(attr.name.unwrap().value(), "field\0名");
    assert!(attr.get);
    assert!(!attr.set);
  }

  #[test]
  fn field_attributes_merge_without_changing_name() {
    let field: Field = parse_quote! {
      #[lua(name = r#"quoted "field""#)]
      #[lua(get)]
      #[lua(set)]
      value: u32
    };
    let attr = parse_field_lua_attr(&field.attrs).unwrap();
    assert_eq!(attr.name.unwrap().value(), "quoted \"field\"");
    assert!(attr.get && attr.set);
  }

  /// 无值开关（`skip` / `get` / `set`）被写成 `flag = value` 时必须报错，
  /// 不能静默丢弃赋值语义。
  #[test]
  fn switch_flags_reject_assigned_value() {
    let fields: [Field; 3] = [
      parse_quote! { #[lua(skip = true)] value: u32 },
      parse_quote! { #[lua(get = 1)] value: u32 },
      parse_quote! { #[lua(set = "x")] value: u32 },
    ];
    let expected = [
      "`skip` does not take a value",
      "`get` does not take a value",
      "`set` does not take a value",
    ];
    for (field, want) in fields.iter().zip(expected) {
      let err = parse_field_lua_attr(&field.attrs).err().unwrap();
      assert_eq!(err.to_string(), want);
    }
  }
}
