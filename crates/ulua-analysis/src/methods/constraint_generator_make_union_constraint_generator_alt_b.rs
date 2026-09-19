//! @interface-stub
use alloc::vec::Vec;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    constraint_generator::ConstraintGenerator, union_builder::UnionBuilder, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};
impl ConstraintGenerator {
  pub fn make_union_vector_type_id(&mut self, options: Vec<TypeId>) -> TypeId {
    let mut ub = UnionBuilder::new(self.arena, self.builtin_types);
    ub.reserve(options.len());

    for option in options {
      ub.add(option);
    }

    let union_ty = ub.build();

    if get_type_id::<UnionType>(union_ty).is_some() {
      self.unions_to_simplify.push(union_ty);
    }

    union_ty
  }
}
