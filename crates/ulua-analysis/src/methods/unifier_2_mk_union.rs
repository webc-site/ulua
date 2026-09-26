use crate::{
  functions::{follow_type, simplify_union::simplify_union},
  records::{arena_handle::Handle, unifier_2::Unifier2},
  type_aliases::type_id::TypeId,
};

impl Unifier2 {
  pub fn mk_union(&mut self, left: TypeId, right: TypeId) -> TypeId {
    let left = follow_type::follow(left);
    let right = follow_type::follow(right);
    simplify_union(
      Handle::from_nonnull(self.builtin_types),
      Handle::from_nonnull(self.arena),
      left,
      right,
    )
    .result
  }
}
