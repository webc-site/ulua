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

impl FileResolver for NaiveFileResolver {
  /// 承 `NullFileResolver::readSource`：恒返回 `None`。
  fn read_source(&mut self, _name: &ModuleName) -> Option<SourceCode> {
    None
  }

  /// C++ `NaiveFileResolver::resolveModule` 覆写。
  fn resolve_module(
    &mut self,
    context: Option<&ModuleInfo>,
    expr: &AstExpr,
    _limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    naive_file_resolver_resolve_module_impl(context, expr)
  }
}
