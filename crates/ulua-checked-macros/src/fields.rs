//! `ulua! { .. }` / `ulua_file! { .. }` 共用的 `key = value` 字段解析。
//!
//! 两种宏的字段集合一致，仅必填字段名不同（`source` / `root`），故抽成一处。

use syn::{
  Error, Ident, LitStr, Result, Token, braced, parse::ParseStream, punctuated::Punctuated,
};

use crate::module_entry::ModuleEntry;

/// 两种宏共享的字段集合。
pub(crate) struct CommonFields {
  /// inline 宏的 `source` / file 宏的 `root`（必填）。
  pub source: LitStr,
  pub module: Option<LitStr>,
  pub defs: Option<LitStr>,
  pub modules: Vec<ModuleEntry>,
}

/// 解析 `key = value` 序列；`source_key` 是必填字段名（`source` 或 `root`）。
pub(crate) fn parse_kv(input: ParseStream<'_>, source_key: &str) -> Result<CommonFields> {
  let mut source = None;
  let mut module = None;
  let mut defs = None;
  let mut modules = None;

  while !input.is_empty() {
    let key: Ident = input.parse()?;
    input.parse::<Token![=]>()?;

    match key.to_string().as_str() {
      k if k == source_key => assign_once(&mut source, key, input.parse()?)?,
      "module" => assign_once(&mut module, key, input.parse()?)?,
      "defs" => assign_once(&mut defs, key, input.parse()?)?,
      "modules" => {
        if modules.is_some() {
          return Err(Error::new(key.span(), "duplicate `modules` field"));
        }
        let content;
        braced!(content in input);
        modules = Some(
          Punctuated::<ModuleEntry, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect(),
        );
      }
      _ => {
        return Err(Error::new(
          key.span(),
          format!("expected one of `{source_key}`, `module`, `defs`, or `modules`"),
        ));
      }
    }

    if input.peek(Token![,]) {
      input.parse::<Token![,]>()?;
    } else if !input.is_empty() {
      return Err(input.error("expected `,`"));
    }
  }

  Ok(CommonFields {
    source: source
      .ok_or_else(|| input.error(format_args!("missing required `{source_key}` field")))?,
    module,
    defs,
    modules: modules.unwrap_or_default(),
  })
}

fn assign_once(slot: &mut Option<LitStr>, key: Ident, value: LitStr) -> Result<()> {
  if slot.is_none() {
    *slot = Some(value);
    Ok(())
  } else {
    Err(Error::new(key.span(), format!("duplicate `{key}` field")))
  }
}
