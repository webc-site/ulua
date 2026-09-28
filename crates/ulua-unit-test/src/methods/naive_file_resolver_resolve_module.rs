//! Ported from `tests/Frontend.test.cpp`.
//! Source: `tests/Frontend.test.cpp`

use alloc::format;

use ulua_analysis::{records::module_info::ModuleInfo, type_aliases::module_name_type::ModuleName};
use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
    ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
  },
  rtti::ast_node_try_as,
};

use crate::functions::ast_node_ref::{NodePtr as _, as_node_at};

/// C++ `NaiveFileResolver::resolveModule` 的解析主体（由
/// `impl FileResolver for NaiveFileResolver` 调用）。
///
/// cpp 的 `expr->as<AstExprGlobal>()` 一串下转在 Rust 侧走 `ast_node_try_as`
/// （引用进、`Option<&T>` 出）与 [`NodePtr`]（字段指针槽下转）：返回引用的生命周期
/// 全部由入参 `&AstExpr` 的借用给出，不再有 `*const → *mut` 上转、也不需要在
/// `unsafe` 里判空。
pub fn naive_file_resolver_resolve_module_impl(
  context: Option<&ModuleInfo>,
  expr: &AstExpr,
) -> Option<ModuleInfo> {
  // cpp `AstExpr : AstNode` 的 `#[repr(C)]` 首字段即基类子对象，取 `&expr.base`
  // 是安全的上转（无需指针 cast）。
  if let Some(global) = ast_node_try_as::<AstExprGlobal>(&expr.base) {
    if global.name == "Modules" {
      return Some(ModuleInfo {
        name: ModuleName::from("Modules"),
        optional: false,
      });
    }

    if global.name == "game" {
      return Some(ModuleInfo {
        name: ModuleName::from("game"),
        optional: false,
      });
    }
  } else if let Some(index) = ast_node_try_as::<AstExprIndexName>(&expr.base) {
    let context = context?;
    return Some(ModuleInfo {
      name: ModuleName::from(format!(
        "{}/{}",
        context.name,
        index.index.as_str_or_empty()
      )),
      optional: context.optional,
    });
  } else if let Some(call) = ast_node_try_as::<AstExprCall>(&expr.base) {
    let context = context?;

    if call.self_
      && call.args.size >= 1
      && let (Some(index), Some(func)) = (
        as_node_at::<AstExprConstantString, _>(&call.args, 0),
        call.func.as_node::<AstExprIndexName>(),
      )
      && func.index == "GetService"
      && context.name == "game"
    {
      let service_name = index.value.as_str().unwrap_or("");
      return Some(ModuleInfo {
        name: ModuleName::from(format!("game/{service_name}")),
        optional: false,
      });
    }
  }

  None
}
