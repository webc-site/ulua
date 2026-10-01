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

    // `Ident` 直接与 `&str` 比较（内容相等语义），免去每个键先 `to_string()`
    // 再按 `&str` 匹配的一次堆分配；分支次序与原 match 臂一致。
    if key == source_key {
      source = Some(once(&source, &key, input.parse()?)?);
    } else if key == "module" {
      module = Some(once(&module, &key, input.parse()?)?);
    } else if key == "defs" {
      defs = Some(once(&defs, &key, input.parse()?)?);
    } else if key == "modules" {
      let content;
      braced!(content in input);
      let entries: Vec<ModuleEntry> =
        Punctuated::<ModuleEntry, Token![,]>::parse_terminated(&content)?
          .into_iter()
          .collect();
      modules = Some(once(&modules, &key, entries)?);
    } else {
      return Err(Error::new(
        key.span(),
        format!("expected one of `{source_key}`, `module`, `defs`, or `modules`"),
      ));
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

/// 校验字段首次出现（`seen` 为只读入参），返回其值交调用方写入槽位——出参改返回值。
/// 重复出现即在 `key` 的 span 上报 `duplicate` 错误。
fn once<T>(seen: &Option<T>, key: &Ident, value: T) -> Result<T> {
  if seen.is_some() {
    return Err(Error::new(key.span(), format!("duplicate `{key}` field")));
  }
  Ok(value)
}
