//! 多模块 Luau 源的公共类型检查流程：构建模块表、检测根模块重名、
//! 交由 `ulua_rt::check_modules*` 校验。inline / file 两种宏共用。

use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::{module_entry::ModuleEntry, paths};

/// 未显式指定 `module` 时的根模块名。
const DEFAULT_MODULE: &str = "main";

/// 单条 (1,1) 诊断（模块重名检查共用样板：这类问题在 Luau 侧没有真实位置）。
fn diag(module: &str, message: &str) -> Vec<ulua_rt::TypeDiagnostic> {
  vec![ulua_rt::TypeDiagnostic {
    module: Some(module.to_string()),
    line: 1,
    column: 1,
    end_line: 1,
    end_column: 1,
    message: message.to_string(),
    in_definitions: false,
  }]
}

/// 运行多模块检查；`loaded` 是宏展开处**预先解析好**的「模块名 → 源文本」
/// （inline 宏取字面量值，file 宏读磁盘文件）。源文本加载失败在展开处就已带
/// 精确 span 报为编译错误，故本函数只可能因重名或类型检查失败。
pub(crate) fn check(
  root_module: &str,
  root_source: &str,
  loaded: &[(String, String)],
  defs: Option<&str>,
) -> Result<(), Vec<ulua_rt::TypeDiagnostic>> {
  let mut modules: Vec<(&str, &str)> = Vec::with_capacity(loaded.len() + 1);
  modules.push((root_module, root_source));

  for (name, source) in loaded {
    let (name, source) = (name.as_str(), source.as_str());
    if name == root_module {
      return Err(diag(root_module, "module map duplicates the root module"));
    }
    if modules.iter().any(|(seen, _)| *seen == name) {
      return Err(diag(name, "duplicate module name in module map"));
    }
    modules.push((name, source));
  }

  match defs {
    Some(defs) => ulua_rt::check_modules_with_definitions(root_module, &modules, defs),
    None => ulua_rt::check_modules(root_module, &modules),
  }
}

/// 根模块名：显式 `module` 字段的值，未指定时用默认值。
pub(crate) fn root_module_of(module: Option<&LitStr>) -> String {
  module.map_or_else(|| DEFAULT_MODULE.to_string(), |module| module.value())
}

/// inline / file 两种宏共用的派发：无模块映射且未指定根模块名时走单文件
/// `check` / `check_with_definitions`，否则走 [`check`] 多模块流程；
/// `loaded` 为预解析的模块源文本（同 [`check`]）。
pub(crate) fn check_dispatch(
  module: Option<&LitStr>,
  source: &str,
  loaded: &[(String, String)],
  defs: Option<&str>,
) -> Result<(), Vec<ulua_rt::TypeDiagnostic>> {
  if loaded.is_empty() && module.is_none() {
    return match defs {
      Some(defs) => ulua_rt::check_with_definitions(source, defs),
      None => ulua_rt::check(source),
    };
  }

  check(&root_module_of(module), source, loaded, defs)
}

/// file 宏展开：`include_str!` 依赖顺序固定（const 绑定强制纳入 dep-info），
/// 根文件表达式放最后作为宏的值。
pub(crate) fn expand_include_strs(
  root: &LitStr,
  defs: Option<&LitStr>,
  entries: &[ModuleEntry],
) -> TokenStream {
  let root_expr = paths::include_str_expr(root);
  let mut dependencies = entries
    .iter()
    .map(|entry| paths::include_str_expr(&entry.source_or_path))
    .collect::<Vec<_>>();

  if let Some(defs) = defs {
    dependencies.push(paths::include_str_expr(defs));
  }

  quote! {{
      #(const _: &str = #dependencies;)*
      #root_expr
  }}
}
