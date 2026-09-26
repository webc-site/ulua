use std::convert::identity;

use proc_macro2::TokenStream;

use crate::{fields::CommonFields, file_input::FileInput, modules_check, paths, report};

// 主体收口为 `Result`：各失败分支统一以 `TokenStream`（compile_error! 体）作错误型，
// 经 `map_err` 串联后由 `expand` 折叠回 `TokenStream`，消除逐级 `match` 阶梯。
fn expand_inner(tokens: TokenStream) -> Result<TokenStream, TokenStream> {
  let FileInput {
    common: CommonFields {
      source: root,
      module,
      defs,
      modules,
    },
  } = syn::parse2::<FileInput>(tokens).map_err(|err| err.to_compile_error())?;

  let root_source = paths::read_manifest_file(&root).map_err(|err| err.to_compile_error())?;
  let defs_source = defs
    .as_ref()
    .map(paths::read_manifest_file)
    .transpose()
    .map_err(|err| err.to_compile_error())?;
  // 预读所有模块条目：读失败的 `syn::Error` 已带 path 字面量的精确 span，
  // 直接 `to_compile_error` 上抛，不经字符串降级（否则诊断位置会被抹平）。
  let loaded = modules
    .iter()
    .map(|entry| {
      paths::read_manifest_file(&entry.source_or_path).map(|source| (entry.name.value(), source))
    })
    .collect::<syn::Result<Vec<(String, String)>>>()
    .map_err(|err| err.to_compile_error())?;

  let result = modules_check::check_dispatch(
    module.as_ref(),
    &root_source,
    &loaded,
    defs_source.as_deref(),
  );

  if let Err(diagnostics) = result {
    return Err(report::diagnostics_error(root.span(), &diagnostics));
  }

  Ok(modules_check::expand_include_strs(
    &root,
    defs.as_ref(),
    &modules,
  ))
}

pub fn expand(tokens: TokenStream) -> TokenStream {
  expand_inner(tokens).unwrap_or_else(identity)
}
