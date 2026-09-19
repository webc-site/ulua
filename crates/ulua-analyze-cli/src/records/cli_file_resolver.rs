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
}

impl FileResolver for CliFileResolver {
  /// `std::optional<SourceCode> readSource(const ModuleName&)`
  fn read_source(&mut self, name: &ModuleName) -> Option<SourceCode> {
    Self::read_source(self, name)
  }

  /// `std::optional<ModuleInfo> resolveModule(const ModuleInfo*, AstExpr*, const TypeCheckLimits&)`
  fn resolve_module(
    &mut self,
    context: Option<&ModuleInfo>,
    expr: &AstExpr,
    limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    Self::resolve_module(self, context, expr, limits)
  }

  /// `std::string getHumanReadableModuleName(const ModuleName&) const`
  fn get_human_readable_module_name(&self, name: &ModuleName) -> String {
    cli_file_resolver_get_human_readable_module_name(name)
  }
}
