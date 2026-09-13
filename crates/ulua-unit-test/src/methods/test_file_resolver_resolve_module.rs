use alloc::{format, string::ToString};

use ulua_analysis::records::{module_info::ModuleInfo, type_check_limits::TypeCheckLimits};
use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
    ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::records::test_file_resolver::TestFileResolver;

impl TestFileResolver {
  pub fn resolve_module(
    &self,
    context: Option<&ModuleInfo>,
    expr: &AstExpr,
    _limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    let node = expr as *const AstExpr as *mut AstNode;

    unsafe {
      if let Some(global) = ast_node_as::<AstExprGlobal>(node).as_ref() {
        if global.name == "game" {
          return Some(ModuleInfo {
            name: "game".to_string(),
            optional: false,
          });
        }

        if global.name == "workspace" {
          return Some(ModuleInfo {
            name: "workspace".to_string(),
            optional: false,
          });
        }

        if global.name == "script" {
          return context.cloned();
        }
      } else if let Some(index) = ast_node_as::<AstExprIndexName>(node).as_ref() {
        let context = context?;

        if index.index == "Parent" {
          let last_separator = context.name.rfind('/')?;
          return Some(ModuleInfo {
            name: context.name[..last_separator].to_string(),
            optional: context.optional,
          });
        }

        return Some(ModuleInfo {
          name: format!("{}/{}", context.name, index.index.as_str_or_empty()),
          optional: context.optional,
        });
      } else if let Some(index) = ast_node_as::<AstExprIndexExpr>(node).as_ref() {
        let context = context?;
        let index_expr = index.index as *mut AstNode;

        if let Some(index_string) = ast_node_as::<AstExprConstantString>(index_expr).as_ref() {
          let s = index_string.value.as_str().unwrap_or("");
          return Some(ModuleInfo {
            name: format!("{}/{}", context.name, s),
            optional: context.optional,
          });
        }
      } else if let Some(call) = ast_node_as::<AstExprCall>(node).as_ref() {
        let context = context?;

        if call.self_ && call.args.size >= 1 && context.name == "game" {
          let arg = *call.args.data;
          let arg_node = arg as *mut AstNode;
          let func_node = call.func as *mut AstNode;

          if let (Some(index_string), Some(func)) = (
            ast_node_as::<AstExprConstantString>(arg_node).as_ref(),
            ast_node_as::<AstExprIndexName>(func_node).as_ref(),
          ) && func.index == "GetService"
          {
            let service_name = index_string.value.as_str().unwrap_or("");
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
}
