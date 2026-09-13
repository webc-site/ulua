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
  pub(crate) name: Option<String>,
  /// `skip` — do not expose this field to Lua at all.
  pub(crate) skip: bool,
  /// `get` — expose a getter.
  pub(crate) get: bool,
  /// `set` — expose a setter.
  pub(crate) set: bool,
}

impl LuaAttr {
  /// Parse a single nested meta item from `#[lua(...)]`, accumulating into
  /// `self`. Mirrors `mlua_derive`'s `LuaAttr::parse_inner` for the field
  /// flags; the method-only flags are rejected with a helpful message.
  pub(crate) fn parse_inner(&mut self, meta: ParseNestedMeta) -> Result<()> {
    match &meta.path {
      path if path.is_ident("skip") => {
        if meta.value().is_ok() {
          return Err(meta.error("`skip` does not take a value"));
        }
        self.skip = true;
      }
      path if path.is_ident("get") => self.get = true,
      path if path.is_ident("set") => self.set = true,
      path if path.is_ident("name") => {
        let value = meta.value()?;
        let lit: LitStr = value.parse()?;
        self.name = Some(lit.value());
      }
      // The impl-method flags are valid mlua attributes but are only
      // meaningful inside `#[userdata_impl]`, which ulua-rt-derive does
      // not provide. Reject them explicitly rather than silently ignore.
      path
        if path.is_ident("getter")
          || path.is_ident("setter")
          || path.is_ident("field")
          || path.is_ident("meta")
          || path.is_ident("infallible") =>
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
