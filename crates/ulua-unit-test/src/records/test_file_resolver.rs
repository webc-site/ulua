//! Source: `tests/Fixture.h`

use std::{collections::HashMap, sync::Arc};

use ulua_analysis::{
  enums::type_file_resolver::Type as SourceCodeType,
  records::{
    file_resolver::FileResolver, module_info::ModuleInfo, require_suggester::RequireSuggester,
    source_code::SourceCode, type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::records::ast_expr::AstExpr;

use crate::records::test_require_suggester::{
  TestRequireSuggester, wire_require_suggester_sources,
};

#[derive(Debug, Default)]
pub struct TestFileResolver {
  pub source: HashMap<ModuleName, String>,
  pub source_types: HashMap<ModuleName, SourceCodeType>,
  pub environments: HashMap<ModuleName, String>,
  /// C++ `FileResolver::requireSuggester` 成员；经 trait 的
  /// `require_suggester` 读取。
  pub require_suggester: Option<Arc<dyn RequireSuggester>>,
}

impl TestFileResolver {
  pub fn enable_require_suggester(&mut self) {
    wire_require_suggester_sources(&self.source);

    if self.require_suggester.is_none() {
      self.require_suggester = Some(Arc::new(TestRequireSuggester::default()));
    }
  }
}

impl FileResolver for TestFileResolver {
  /// C++ `TestFileResolver::readSource`
  fn read_source(&mut self, name: &ModuleName) -> Option<SourceCode> {
    Self::read_source(self, name)
  }

  /// C++ `TestFileResolver::resolveModule`
  fn resolve_module(
    &mut self,
    context: Option<&ModuleInfo>,
    expr: &AstExpr,
    limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    Self::resolve_module(self, context, expr, limits)
  }

  /// C++ `TestFileResolver::getHumanReadableModuleName`
  fn get_human_readable_module_name(&self, name: &ModuleName) -> String {
    Self::get_human_readable_module_name(self, name)
  }

  /// C++ `TestFileResolver::getEnvironmentForModule`
  fn get_environment_for_module(&self, name: &ModuleName) -> Option<String> {
    Self::get_environment_for_module(self, name)
  }

  fn require_suggester(&self) -> Option<&Arc<dyn RequireSuggester>> {
    self.require_suggester.as_ref()
  }
}
