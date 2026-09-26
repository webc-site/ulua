//! Parsing of `#[lua(...)]` field attributes for `#[derive(UserData)]`.
//!
//! Mirrors the field-context subset of `mlua_derive`'s `LuaAttr`
//! (`mlua_derive/src/userdata/attr.rs`): the flags that apply to **struct
//! fields** — `skip`, `get`, `set`, `name = "..."`. The impl-method-context
//! flags (`getter`, `setter`, `field`, `meta`, `infallible`) belong to mlua's
//! `#[userdata_impl]` attribute macro, which is not part of the derive and is
//! deferred (see crate docs).

use proc_macro2::Span;
use syn::{LitStr, Result, meta::ParseNestedMeta};

/// Parsed `#[lua(...)]` attribute on a struct field.
#[derive(Default)]
pub(crate) struct LuaAttr {
  /// Span of the originating `#[lua(...)]` attribute, for diagnostics.
  pub(crate) span: Option<Span>,
  /// `name = "..."` — the Lua-visible field name (defaults to the Rust ident).
  pub(crate) name: Option<LitStr>,
  /// `skip` — do not expose this field to Lua at all.
  pub(crate) skip: bool,
  /// `get` — expose a getter.
  pub(crate) get: bool,
  /// `set` — expose a setter.
  pub(crate) set: bool,
}

/// 只适用于 impl 方法（mlua 的 `#[userdata_impl]`）的旗标；derive 侧必须显式拒绝，
/// 不能静默当作未知键。
const METHOD_ONLY_FLAGS: [&str; 5] = ["getter", "setter", "field", "meta", "infallible"];

impl LuaAttr {
  /// 无值开关旗标（`skip` / `get` / `set`）的统一解析：写成 `flag = value` 直接报错，
  /// 避免静默丢弃用户本意的赋值。
  fn parse_switch(meta: &ParseNestedMeta, name: &str, slot: &mut bool) -> Result<()> {
    if meta.value().is_ok() {
      return Err(meta.error(format_args!("`{name}` does not take a value")));
    }
    *slot = true;
    Ok(())
  }

  /// Parse a single nested meta item from `#[lua(...)]`, accumulating into
  /// `self`. Mirrors `mlua_derive`'s `LuaAttr::parse_inner` for the field
  /// flags; the method-only flags are rejected with a helpful message.
  pub(crate) fn parse_inner(&mut self, meta: ParseNestedMeta) -> Result<()> {
    match &meta.path {
      path if path.is_ident("skip") => Self::parse_switch(&meta, "skip", &mut self.skip)?,
      path if path.is_ident("get") => Self::parse_switch(&meta, "get", &mut self.get)?,
      path if path.is_ident("set") => Self::parse_switch(&meta, "set", &mut self.set)?,
      path if path.is_ident("name") => {
        let value = meta.value()?;
        self.name = Some(value.parse()?);
      }
      // The impl-method flags are valid mlua attributes but are only
      // meaningful inside `#[userdata_impl]`, which ulua-rt-derive does
      // not provide. Reject them explicitly rather than silently ignore.
      path
        if METHOD_ONLY_FLAGS
          .iter()
          .copied()
          .any(|flag| path.is_ident(flag)) =>
      {
        return Err(meta.error(
          "this `#[lua(...)]` flag only applies to impl methods \
                     (mlua's `#[userdata_impl]`), which ulua-rt-derive does not \
                     implement; on a struct field use `skip`, `get`, `set`, `name`",
        ));
      }
      _ => {
        return Err(
          meta.error("unsupported lua attribute, expected: `skip`, `get`, `set`, `name`"),
        );
      }
    }
    Ok(())
  }

  /// Span to use for diagnostics (falls back to the call site).
  pub(crate) fn span(&self) -> Span {
    self.span.unwrap_or_else(Span::call_site)
  }
}
