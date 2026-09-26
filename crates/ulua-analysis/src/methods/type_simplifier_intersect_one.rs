use crate::{
  enums::relation::Relation, functions::relate_simplify::relate_type_id_type_id,
  records::type_simplifier::TypeSimplifier, type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_one(&self, target: TypeId, discriminant: TypeId) -> Option<TypeId> {
    let builtin_types = self.builtin_types.get();
    match relate_type_id_type_id(target, discriminant) {
      Relation::Disjoint => Some(builtin_types.never_type),
      Relation::Subset | Relation::Coincident => Some(target),
      Relation::Superset => Some(discriminant),
      Relation::Intersects => None,
    }
  }
}
