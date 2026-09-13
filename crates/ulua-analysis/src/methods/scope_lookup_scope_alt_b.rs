use crate::{
  records::scope::Scope,
  type_aliases::{def_id_def::DefId, type_id::TypeId},
};

impl Scope {
  /// `std::optional<TypeId> Scope::lookup(DefId def) const` (Scope.cpp:98-110).
  pub fn lookup_def_id(&self, def: DefId) -> Option<TypeId> {
    let mut current: Option<&Scope> = Some(self);
    while let Some(scope) = current {
      if let Some(ty) = scope.rvalue_refinements.find(&def) {
        return Some(*ty);
      }
      if let Some(ty) = scope.lvalue_types.find(&def) {
        return Some(*ty);
      }

      current = scope.parent.as_ref().map(|scoped| scoped.as_ref());
    }

    None
  }
}
