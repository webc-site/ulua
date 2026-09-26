//! Source: `Analysis/src/Module.cpp:355-359`

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::module::Module, type_aliases::scope_ptr_type::ScopePtr};

impl Module {
  /// `ScopePtr Module::getModuleScope() const`.
  /// Reference: `Module.cpp:355-359`.
  pub fn get_module_scope(&self) -> ScopePtr {
    LUAU_ASSERT!(self.has_module_scope());
    // C++: return scopes.front().second;
    // 紧邻 LUAU_ASSERT(has_module_scope()) 即 `!scopes.is_empty()`，first() 必命中。
    self
      .scopes
      .first()
      .expect("紧邻 LUAU_ASSERT(has_module_scope()) 蕴含非空")
      .1
      .clone()
  }
}
