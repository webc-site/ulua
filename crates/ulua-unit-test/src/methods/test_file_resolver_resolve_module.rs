use alloc::format;

use ulua_analysis::{
  records::{module_info::ModuleInfo, type_check_limits::TypeCheckLimits},
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
    ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
  },
  rtti::ast_node_try_as,
};

use crate::{
  functions::ast_node_ref::{NodePtr as _, as_node_at},
  records::test_file_resolver::TestFileResolver,
};

impl TestFileResolver {
  /// cpp `TestFileResolver::resolveModule`（`Fixture.cpp:177-215`）：按 `expr->as<T>()`
  /// 的四路下转分派。Rust 侧统一走 `ast_node_try_as`（引用进、`Option<&T>` 出）与
  /// [`NodePtr`]（字段指针槽下转），返回引用的生命周期由入参 `&AstExpr` 给出，
  /// 不再有 `*const → *mut` 上转与手工判空。
  pub fn resolve_module(
    &self,
    context: Option<&ModuleInfo>,
    expr: &AstExpr,
    _limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    // cpp `AstExpr : AstNode` 是 `#[repr(C)]` 单继承，`base` 即偏移 0 的基类子对象。
    if let Some(global) = ast_node_try_as::<AstExprGlobal>(&expr.base) {
      if global.name == "game" {
        return Some(ModuleInfo {
          name: ModuleName::from("game"),
          optional: false,
        });
      }

      if global.name == "workspace" {
        return Some(ModuleInfo {
          name: ModuleName::from("workspace"),
          optional: false,
        });
      }

      if global.name == "script" {
        return context.cloned();
      }
    } else if let Some(index) = ast_node_try_as::<AstExprIndexName>(&expr.base) {
      let context = context?;

      if index.index == "Parent" {
        let last_separator = context.name.rfind('/')?;
        return Some(ModuleInfo {
          name: ModuleName::from(&context.name[..last_separator]),
          optional: context.optional,
        });
      }

      return Some(ModuleInfo {
        name: ModuleName::from(format!(
          "{}/{}",
          context.name,
          index.index.as_str_or_empty()
        )),
        optional: context.optional,
      });
    } else if let Some(index) = ast_node_try_as::<AstExprIndexExpr>(&expr.base) {
      let context = context?;

      if let Some(index_string) = index.index.as_node::<AstExprConstantString>() {
        let s = index_string.value.as_str().unwrap_or("");
        return Some(ModuleInfo {
          name: ModuleName::from(format!("{}/{}", context.name, s)),
          optional: context.optional,
        });
      }
    } else if let Some(call) = ast_node_try_as::<AstExprCall>(&expr.base) {
      let context = context?;

      if call.self_
        && call.args.size >= 1
        && context.name == "game"
        && let (Some(index_string), Some(func)) = (
          as_node_at::<AstExprConstantString, _>(&call.args, 0),
          call.func.as_node::<AstExprIndexName>(),
        )
        && func.index == "GetService"
      {
        let service_name = index_string.value.as_str().unwrap_or("");
        return Some(ModuleInfo {
          name: ModuleName::from(format!("game/{service_name}")),
          optional: false,
        });
      }
    }

    None
  }
}
