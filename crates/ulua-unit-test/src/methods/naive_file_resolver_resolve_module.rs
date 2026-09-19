//! Ported from `tests/Frontend.test.cpp`.
//! Source: `tests/Frontend.test.cpp`

use alloc::{format, string::String};

use ulua_analysis::records::module_info::ModuleInfo;
use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
    ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

/// C++ `NaiveFileResolver::resolveModule` 的解析主体（由
/// `impl FileResolver for NaiveFileResolver` 调用）。
pub fn naive_file_resolver_resolve_module_impl(
  context: Option<&ModuleInfo>,
  expr: &AstExpr,
) -> Option<ModuleInfo> {
  let node = expr as *const AstExpr as *mut AstNode;

  unsafe {
    if let Some(global) = ast_node_as::<AstExprGlobal>(node).as_ref() {
      if global.name == "Modules" {
        return Some(ModuleInfo {
          name: String::from("Modules"),
          optional: false,
        });
      }

      if global.name == "game" {
        return Some(ModuleInfo {
          name: String::from("game"),
          optional: false,
        });
      }
    } else if let Some(index) = ast_node_as::<AstExprIndexName>(node).as_ref() {
      let context = context?;
      return Some(ModuleInfo {
        name: format!("{}/{}", context.name, index.index.as_str_or_empty()),
        optional: context.optional,
      });
    } else if let Some(call) = ast_node_as::<AstExprCall>(node).as_ref() {
      let context = context?;

      if call.self_ && call.args.size >= 1 {
        let arg = *call.args.data;
        let arg_node = arg as *mut AstNode;
        let func_node = call.func as *mut AstNode;

        if let (Some(index), Some(func)) = (
          ast_node_as::<AstExprConstantString>(arg_node).as_ref(),
          ast_node_as::<AstExprIndexName>(func_node).as_ref(),
        ) && func.index == "GetService"
          && context.name == "game"
        {
          let service_name = index.value.as_str().unwrap_or("");
          return Some(ModuleInfo {
            name: format!("game/{service_name}"),
            optional: false,
          });
        }
      }
    }
  }

  None
}
