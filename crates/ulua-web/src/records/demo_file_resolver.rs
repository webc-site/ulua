//! Port of `DemoFileResolver : Luau::FileResolver` (`CLI/src/Web.cpp:16-46`).
//!
//! The luau.org/demo file resolver. Like the project's `NullFileResolver`, it is
//! a concrete `FileResolver` implementor: the four virtuals become trait methods
//! delegating to the inherent methods in `methods/`.
//!
//! C++ member: `std::unordered_map<ModuleName, std::string> source;` — ported as
//! a typed `HashMap<ModuleName, String>` (no untyped JSON).

use alloc::string::String;
use std::collections::HashMap;

use ulua_analysis::{
  records::{
    file_resolver::FileResolver, module_info::ModuleInfo, source_code::SourceCode,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::records::ast_expr::AstExpr;

#[derive(Debug)]
pub struct DemoFileResolver {
  pub source: HashMap<ModuleName, String>,
}

impl Default for DemoFileResolver {
  fn default() -> Self {
    Self::new()
  }
}

impl DemoFileResolver {
  pub fn new() -> Self {
    DemoFileResolver {
      source: HashMap::new(),
    }
  }
}

impl FileResolver for DemoFileResolver {
  /// `std::optional<SourceCode> readSource(const ModuleName&)`
  fn read_source(&mut self, name: &ModuleName) -> Option<SourceCode> {
    Self::read_source(self, name)
  }

  /// `std::optional<ModuleInfo> resolveModule(const ModuleInfo*, AstExpr*, const TypeCheckLimits&)`
  fn resolve_module(
    &mut self,
    context: *const ModuleInfo,
    expr: *mut AstExpr,
    limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    Self::resolve_module(self, context, expr, limits)
  }

  /// `std::string getHumanReadableModuleName(const ModuleName&) const`
  fn get_human_readable_module_name(&self, name: &ModuleName) -> String {
    Self::get_human_readable_module_name(self, name)
  }

  /// `std::optional<std::string> getEnvironmentForModule(const ModuleName&) const`
  fn get_environment_for_module(&self, name: &ModuleName) -> Option<String> {
    Self::get_environment_for_module(self, name)
  }
}
