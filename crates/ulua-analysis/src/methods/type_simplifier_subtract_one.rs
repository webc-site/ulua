use crate::{
  enums::relation::Relation,
  functions::{follow_type, get_type, relate_simplify::relate_type_id_type_id},
  records::{negation_type::NegationType, type_simplifier::TypeSimplifier},
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn subtract_one(&self, target: TypeId, discriminant: TypeId) -> Option<TypeId> {
    let builtin_types = self.builtin_types.get();
    let target = follow_type::follow(target);
    let discriminant = follow_type::follow(discriminant);
    if let Some(nt) = get_type::get::<NegationType>(discriminant) {
      return self.intersect_one(target, nt.ty);
    }
    match relate_type_id_type_id(target, discriminant) {
      Relation::Disjoint => Some(target),
      Relation::Subset | Relation::Coincident => Some(builtin_types.never_type),
      _ => None,
    }
  }
}
