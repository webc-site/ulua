//! 多模块 Luau 源的公共类型检查流程：构建模块表、检测根模块重名、
//! 交由 `ulua_rt::check_modules*` 校验。inline / file 两种宏共用。

use core::result::Result;

use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::{module_entry::ModuleEntry, paths};

/// 未显式指定 `module` 时的根模块名。
const DEFAULT_MODULE: &str = "main";

/// 单条 (1,1) 诊断（根模块重名 / 加载失败共用样板）。
fn diag(module: &str, message: String) -> Vec<ulua_rt::TypeDiagnostic> {
  vec![ulua_rt::TypeDiagnostic {
    module: Some(module.to_string()),
    line: 1,
    column: 1,
    end_line: 1,
    end_column: 1,
    message,
    in_definitions: false,
  }]
}

/// 运行多模块检查；`load` 决定模块条目的源文本来源
/// （inline 宏直接取字面量值，file 宏读磁盘文件）。
pub(crate) fn check(
  root_module: String,
  root_source: &str,
  entries: &[ModuleEntry],
  defs: Option<&str>,
  mut load: impl FnMut(&LitStr) -> Result<String, String>,
) -> Result<(), Vec<ulua_rt::TypeDiagnostic>> {
  let mut modules = Vec::with_capacity(entries.len() + 1);
  modules.push((root_module.clone(), root_source.to_string()));

  for entry in entries {
    let name = entry.name.value();
    if name == root_module {
      return Err(diag(
        &root_module,
        "module map duplicates the root module".to_string(),
      ));
    }
    let source = load(&entry.source_or_path).map_err(|message| diag(&name, message))?;
    modules.push((name, source));
  }

  let borrowed: Vec<(&str, &str)> = modules
    .iter()
    .map(|(name, source)| (name.as_str(), source.as_str()))
    .collect();

  match defs {
    Some(defs) => ulua_rt::check_modules_with_definitions(&root_module, &borrowed, defs),
    None => ulua_rt::check_modules(&root_module, &borrowed),
  }
}

/// 未显式指定根模块名时的默认值。
pub(crate) fn default_module() -> String {
  DEFAULT_MODULE.to_string()
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
