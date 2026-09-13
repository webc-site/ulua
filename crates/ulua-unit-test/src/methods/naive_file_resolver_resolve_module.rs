//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Method:Luau.UnitTest:tests/Frontend.test.cpp:30:naive_file_resolver_resolve_module`
//! Source: `tests/Frontend.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Frontend.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//! - incoming:
//!   - declares <- source_file tests/Frontend.test.cpp
//!   - type_ref <- method TestFileResolver::resolveModule (tests/Fixture.cpp)
//! - outgoing:
//!   - type_ref -> record ModuleInfo (Analysis/include/Luau/FileResolver.h)
//!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
//!   - type_ref -> record TypeCheckLimits (Analysis/include/Luau/TypeCheckLimits.h)
//!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record NaiveFileResolver (tests/Frontend.test.cpp)
//!   - translates_to -> rust_item NaiveFileResolver::resolveModule

use alloc::{format, string::String};

use ulua_analysis::records::{
  file_resolver::FileResolver, module_info::ModuleInfo, type_check_limits::TypeCheckLimits,
};
use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
    ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::records::naive_file_resolver::NaiveFileResolver;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn naive_file_resolver_resolve_module_vtable(
  this: *mut FileResolver,
  context: *const ModuleInfo,
  expr: *mut AstExpr,
  limits: &TypeCheckLimits,
) -> Option<ModuleInfo> {
  if expr.is_null() {
    return None;
  }

  let resolver = this as *const NaiveFileResolver;
  let context = if context.is_null() {
    None
  } else {
    Some(unsafe { &*context })
  };

  unsafe { (*resolver).resolve_module(context, &*expr, limits) }
}

impl NaiveFileResolver {
  pub fn resolve_module(
    &self,
    context: Option<&ModuleInfo>,
    expr: &AstExpr,
    _limits: &TypeCheckLimits,
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
}
