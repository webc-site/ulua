use ulua_ast::enums::mode::Mode;

use crate::{
  records::{source_module::SourceModule, type_checker::TypeChecker},
  type_aliases::{module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr},
};

impl TypeChecker {
  pub fn check_source_module_mode_optional_scope_ptr(
    &mut self,
    module: &SourceModule,
    mode: Mode,
    environment_scope: Option<ScopePtr>,
  ) -> ModulePtr {
    self.check_without_recursion_check(module, mode, environment_scope)
  }
}
