use crate::{
  enums::relation::Relation,
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    relate_simplify_alt_b::relate_type_id_type_id,
  },
  records::{
    intersection_type::IntersectionType, negation_type::NegationType,
    type_simplifier::TypeSimplifier, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_negations(&mut self, left: TypeId, right: TypeId) -> TypeId {
    // C++ Simplify.cpp intersectNegations: LUAU_ASSERT(leftNegation/rightNegation)
    let left_negation = get_type_id::<NegationType>(left).expect("left is NegationType");
    if get_type_id::<UnionType>(follow_type_id(left_negation.ty)).is_some() {
      return self.intersect_negated_union(left, right);
    }
    let right_negation = get_type_id::<NegationType>(right).expect("right is NegationType");
    if get_type_id::<UnionType>(follow_type_id(right_negation.ty)).is_some() {
      return self.intersect_negated_union(right, left);
    }
    match relate_type_id_type_id(left_negation.ty, right_negation.ty) {
      // ~true & ~true
      Relation::Coincident | Relation::Superset => left,
      // ~true & ~boolean
      Relation::Subset => right,
      // ~boolean & ~string
      _ => {
        let arena = unsafe { &mut *self.arena.cast_mut() };
        arena.add_type(IntersectionType {
          parts: alloc::vec![left, right],
        })
      }
    }
  }
}
