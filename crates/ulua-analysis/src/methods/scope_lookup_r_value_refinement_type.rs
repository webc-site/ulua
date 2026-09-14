use crate::{
  records::scope::Scope,
  type_aliases::{def_id_def::DefId, type_id::TypeId},
};

impl Scope {
  /// `std::optional<TypeId> Scope::lookupRValueRefinementType(DefId def) const`
  /// (Scope.cpp:87-96).
  pub fn lookup_r_value_refinement_type(&self, def: DefId) -> Option<TypeId> {
    let mut current: Option<&Scope> = Some(self);
    while let Some(scope) = current {
      if let Some(ty) = scope.rvalue_refinements.find(&def) {
        return Some(*ty);
      }

      current = scope.parent.as_ref().map(|scoped| scoped.as_ref());
    }

    None
  }
}
