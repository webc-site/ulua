use crate::{
  enums::relation::Relation,
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    relate_simplify_alt_b::relate_type_id_type_id,
  },
  records::{negation_type::NegationType, type_simplifier::TypeSimplifier},
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn subtract_one(&self, target: TypeId, discriminant: TypeId) -> Option<TypeId> {
    let builtin_types = unsafe { &*self.builtin_types };
    let target = follow_type_id(target);
    let discriminant = follow_type_id(discriminant);
    if let Some(nt) = get_type_id::<NegationType>(discriminant) {
      return self.intersect_one(target, nt.ty);
    }
    match relate_type_id_type_id(target, discriminant) {
      Relation::Disjoint => Some(target),
      Relation::Subset | Relation::Coincident => Some(builtin_types.never_type),
      _ => None,
    }
  }
}
