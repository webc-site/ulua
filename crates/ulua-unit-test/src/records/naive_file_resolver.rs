use ulua_analysis::{
  records::{
    file_resolver::FileResolver, module_info::ModuleInfo, source_code::SourceCode,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::records::ast_expr::AstExpr;

use crate::methods::naive_file_resolver_resolve_module::naive_file_resolver_resolve_module_impl;

/// C++ `struct NaiveFileResolver : NullFileResolver`（tests/Frontend.test.cpp）：
/// 仅覆写 `resolveModule`，其余沿承 `NullFileResolver` 的行为。
#[derive(Debug, Default)]
pub struct NaiveFileResolver;

impl NaiveFileResolver {
  /// C++ `resolveModule` 的裸指针契约适配；公开 trait 方法只做纯转发。
  pub(crate) fn resolve_module_thunk(
    &self,
    context: *const ModuleInfo,
    expr: *mut AstExpr,
  ) -> Option<ModuleInfo> {
    if expr.is_null() {
      return None;
    }

    // SAFETY: `expr` 指向解析中 AST 的存活节点（C++ resolveModule 契约）。
    let context = if context.is_null() {
      None
    } else {
      Some(unsafe { &*context })
    };

    naive_file_resolver_resolve_module_impl(context, unsafe { &*expr })
  }
}

impl FileResolver for NaiveFileResolver {
  /// 承 `NullFileResolver::readSource`：恒返回 `None`。
  fn read_source(&mut self, _name: &ModuleName) -> Option<SourceCode> {
    None
  }

  /// C++ `NaiveFileResolver::resolveModule` 覆写。
  fn resolve_module(
    &mut self,
    context: *const ModuleInfo,
    expr: *mut AstExpr,
    _limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    Self::resolve_module_thunk(self, context, expr)
  }
}
