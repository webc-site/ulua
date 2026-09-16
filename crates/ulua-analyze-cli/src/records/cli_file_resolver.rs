//! Port of `struct CliFileResolver : Luau::FileResolver` (`CLI/src/Analyze.cpp:160-229`).
//!
//! A concrete `FileResolver` implementor: the four virtuals become trait methods
//! delegating to the inherent methods in `methods/` (`getEnvironmentForModule`
//! is not overridden and keeps the trait default, mirroring the C++ base).

use ulua_analysis::{
  records::{
    file_resolver::FileResolver, module_info::ModuleInfo, source_code::SourceCode,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::records::ast_expr::AstExpr;

use crate::methods::cli_file_resolver_get_human_readable_module_name::cli_file_resolver_get_human_readable_module_name;

#[derive(Debug, Default)]
pub struct CliFileResolver;

impl CliFileResolver {
  pub fn new() -> Self {
    Self
  }

  /// C++ `resolveModule` 的裸指针契约适配；公开 trait 方法只做纯转发。
  /// (clippy::not_unsafe_ptr_arg_deref 不允许在公开安全函数体内解引用裸指针)
  pub(crate) fn resolve_module_thunk(
    &mut self,
    context: *const ModuleInfo,
    expr: *mut AstExpr,
    limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    // SAFETY: 固有方法要求 C++ 调用契约（expr 指向存活 AST 节点）。
    unsafe { Self::resolve_module(self, context, expr, limits) }
  }
}

impl FileResolver for CliFileResolver {
  /// `std::optional<SourceCode> readSource(const ModuleName&)`
  fn read_source(&mut self, name: &ModuleName) -> Option<SourceCode> {
    // SAFETY: 固有方法要求 C++ 调用契约（读 stdin / 文件系统）。
    unsafe { Self::read_source(self, name) }
  }

  /// `std::optional<ModuleInfo> resolveModule(const ModuleInfo*, AstExpr*, const TypeCheckLimits&)`
  fn resolve_module(
    &mut self,
    context: *const ModuleInfo,
    expr: *mut AstExpr,
    limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    Self::resolve_module_thunk(self, context, expr, limits)
  }

  /// `std::string getHumanReadableModuleName(const ModuleName&) const`
  fn get_human_readable_module_name(&self, name: &ModuleName) -> String {
    cli_file_resolver_get_human_readable_module_name(name)
  }
}
