use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::type_checker::TypeChecker,
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn get_index_type_from_type(
    &mut self,
    _scope: ScopePtr,
    _type: TypeId,
    _name: &Name,
    _location: &Location,
    _add_errors: bool,
  ) -> Option<TypeId> {
    let error_count = self.current_module.as_ref().expect("current_module 由 check_without_recursion_check 入口置入 Some、末尾才 take()，check 调用树内恒为 Some").errors.len();
    let result = self.get_index_type_from_type_impl(_scope, _type, _name, _location, _add_errors);
    if !_add_errors {
      LUAU_ASSERT!(error_count == self.current_module.as_ref().expect("current_module 由 check_without_recursion_check 入口置入 Some、末尾才 take()，check 调用树内恒为 Some").errors.len());
    }
    result
  }
}
