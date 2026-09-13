//! @interface-stub
use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::{
  records::{
    constraint_generator::ConstraintGenerator, reduce_constraint::ReduceConstraint,
    type_function::TypeFunction,
  },
  type_aliases::{
    constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl ConstraintGenerator {
  pub fn create_type_function_instance(
    &mut self,
    function: &TypeFunction,
    type_arguments: Vec<TypeId>,
    pack_arguments: Vec<TypePackId>,
    scope: &ScopePtr,
    location: Location,
  ) -> TypeId {
    let result = unsafe {
      (*self.arena).add_type_function_type_function_vector_type_id_vector_type_pack_id(
        function,
        type_arguments,
        pack_arguments,
      )
    };

    self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      location,
      ConstraintV::Reduce(ReduceConstraint { ty: result }),
    );

    result
  }
}
