use crate::{
  functions::{follow_type::follow_type_id, is_nil::is_nil},
  records::constraint_generator::ConstraintGenerator,
  type_aliases::type_id::TypeId,
};

impl ConstraintGenerator {
  pub fn is_shared_refinement_assignment_type(&self, ty: TypeId) -> bool {
    let is_nil_assignment = |ty: TypeId| {
      let ty = follow_type_id(ty);
      is_nil(ty)
    };

    if is_nil_assignment(ty) {
      return true;
    }

    self
      .local_types
      .find(&ty)
      .is_some_and(|types| types.begin().into_iter().any(is_nil_assignment))
  }
}
