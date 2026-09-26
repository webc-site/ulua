use crate::{
  enums::relation::Relation,
  functions::{follow_type, get_type, relate_simplify::relate_type_id_type_id},
  records::{
    intersection_type::IntersectionType, negation_type::NegationType,
    type_simplifier::TypeSimplifier, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_negations(&mut self, left: TypeId, right: TypeId) -> TypeId {
    // C++ Simplify.cpp intersectNegations: LUAU_ASSERT(leftNegation/rightNegation)
    let left_negation = get_type::get::<NegationType>(left).expect("left is NegationType");
    if get_type::get::<UnionType>(follow_type::follow(left_negation.ty)).is_some() {
      return self.intersect_negated_union(left, right);
    }
    let right_negation = get_type::get::<NegationType>(right).expect("right is NegationType");
    if get_type::get::<UnionType>(follow_type::follow(right_negation.ty)).is_some() {
      return self.intersect_negated_union(right, left);
    }
    match relate_type_id_type_id(left_negation.ty, right_negation.ty) {
      // ~true & ~true
      Relation::Coincident | Relation::Superset => left,
      // ~true & ~boolean
      Relation::Subset => right,
      // ~boolean & ~string
      _ => {
        let arena = self.arena.get_mut();
        arena.add_type(IntersectionType {
          parts: alloc::vec![left, right],
        })
      }
    }
  }
}
