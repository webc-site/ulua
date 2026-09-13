use crate::{
  enums::relation::Relation,
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    relate_simplify_alt_b::relate_type_id_type_id,
  },
  records::{
    negation_type::NegationType, never_type::NeverType, type_ids::TypeIds,
    type_simplifier::TypeSimplifier, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_negated_union(&mut self, left: TypeId, right: TypeId) -> TypeId {
    let builtin_types = unsafe { &*self.builtin_types };
    // C++ Simplify.cpp intersectNegatedUnion: LUAU_ASSERT(leftNegation/negatedUnion)
    let left_negation = get_type_id::<NegationType>(left).expect("left is NegationType");
    let negated_ty = follow_type_id(left_negation.ty);
    let negated_union = get_type_id::<UnionType>(negated_ty).expect("negated is UnionType");

    let mut changed = false;
    let mut new_parts = TypeIds::new();
    for &part in &negated_union.options {
      match relate_type_id_type_id(part, right) {
        // If A is disjoint from B, then ~A & B is just B.
        //
        // ~(false?) & true
        // (~false & true) & (~nil & true)
        // true & true
        Relation::Disjoint => new_parts.insert_type_id(right),
        // If A is coincident with or a superset of B, then ~A & B is never.
        //
        // ~(false?) & false
        // (~false & false) & (~nil & false)
        // never & false
        //
        // fallthrough
        //
        // ~(boolean | nil) & true
        // (~boolean & true) & (~boolean & nil)
        // never & nil
        Relation::Coincident | Relation::Superset => return builtin_types.never_type,
        // If A is a subset of B, then ~A & B is a bit more complicated.
        // We need to think harder.
        //
        // ~(false?) & boolean
        // (~false & boolean) & (~nil & boolean)
        // true & boolean
        Relation::Subset | Relation::Intersects => {
          let simplified = self.intersect_type_with_negation(self.mk_negation(part), right);
          changed |= simplified != right;
          if get_type_id::<NeverType>(simplified).is_some() {
            changed = true;
          } else {
            new_parts.insert_type_id(simplified);
          }
        }
      }
    }
    if !changed {
      return right;
    }
    self.intersect_from_parts(new_parts)
  }
}
