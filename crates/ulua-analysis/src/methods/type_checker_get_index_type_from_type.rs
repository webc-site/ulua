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
    let error_count = self.expect_current_module().errors.len();
    let result = self.get_index_type_from_type_impl(_scope, _type, _name, _location, _add_errors);
    if !_add_errors {
      LUAU_ASSERT!(error_count == self.expect_current_module().errors.len());
    }
    result
  }
}
