use ulua_ast::records::location::Location;

use crate::{
  records::constraint_generator::ConstraintGenerator,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl ConstraintGenerator {
  pub fn make_intersect(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    lhs: TypeId,
    rhs: TypeId,
  ) -> TypeId {
    let builtin_types = unsafe { &*self.builtin_types };
    let intersect_func = &builtin_types.type_functions.intersect_func;

    self.create_type_function_instance(
      intersect_func,
      alloc::vec![lhs, rhs],
      alloc::vec![],
      scope,
      location,
    )
  }
}
