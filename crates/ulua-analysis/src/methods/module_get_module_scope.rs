//! Node: `cxx:Method:Luau.Analysis:Analysis/src/Module.cpp:355:module_get_module_scope`
//! Source: `Analysis/src/Module.cpp:355-359`

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::module::Module, type_aliases::scope_ptr_type::ScopePtr};

impl Module {
  /// `ScopePtr Module::getModuleScope() const`.
  /// Reference: `Module.cpp:355-359`.
  pub fn get_module_scope(&self) -> ScopePtr {
    LUAU_ASSERT!(self.has_module_scope());
    // C++: return scopes.front().second;
    self.scopes.first().unwrap().1.clone()
  }
}
