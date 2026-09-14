use crate::{
  functions::{follow_type::follow_type_id, simplify_union::simplify_union},
  records::unifier_2::Unifier2,
  type_aliases::type_id::TypeId,
};

impl Unifier2 {
  pub fn mk_union(&mut self, left: TypeId, right: TypeId) -> TypeId {
    let left = follow_type_id(left);
    let right = follow_type_id(right);
    let builtin_types = self.builtin_types.as_ptr();
    let arena = self.arena.as_ptr();
    simplify_union(builtin_types, arena, left, right).result
  }
}
