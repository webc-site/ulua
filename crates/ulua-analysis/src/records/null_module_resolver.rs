//! Source: `Analysis/include/Luau/ModuleResolver.h`
//!
//! C++ `struct NullModuleResolver : ModuleResolver` (ModuleResolver.h:51-69):
//! a stateless resolver that answers "module unknown" to every query. It has
//! no data members; the `ModuleResolver` interface overrides live as
//! `NullModuleResolver` methods (see `methods/null_module_resolver_*`).

use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  records::{module_info::ModuleInfo, module_resolver::ModuleResolver},
  type_aliases::{module_name_type::ModuleName, module_ptr_module::ModulePtr},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct NullModuleResolver;

impl NullModuleResolver {
  /// Construct the (stateless) null resolver.
  pub fn new() -> Self {
    NullModuleResolver
  }
}

/// C++ 虚函数覆盖的 Rust 侧承接：全部回答"模块未知"。
impl ModuleResolver for NullModuleResolver {
  fn resolve_module_info(
    &self,
    _current_module_name: &ModuleName,
    _path_expr: &AstExpr,
  ) -> Option<ModuleInfo> {
    None
  }

  fn get_module(&self, _module_name: &ModuleName) -> Option<ModulePtr> {
    None
  }

  fn module_exists(&self, _module_name: &ModuleName) -> bool {
    false
  }

  fn get_human_readable_module_name(&self, module_name: &ModuleName) -> String {
    module_name.clone()
  }
}
