use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::{
  records::{constraint::Constraint, constraint_generator::ConstraintGenerator},
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr},
};
impl ConstraintGenerator {
  pub fn add_constraint_scope_ptr_location_constraint_v(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    cv: ConstraintV,
  ) -> *mut Constraint {
    let c = Box::new(Constraint {
      scope: scope.as_ref() as *const _ as *mut _,
      location,
      c: cv,
      deprecated_dependencies: Vec::new(),
    });
    let c_ptr = Box::into_raw(c);
    self.constraints.push(c_ptr);
    c_ptr
  }
}
