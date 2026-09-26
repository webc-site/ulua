use crate::{
  records::{scope::Scope, scope_registry::resolve_scope, symbol::Symbol},
  type_aliases::{def_id_def::DefId, type_id::TypeId},
};

impl Scope {
  pub fn lookup_symbol(&self, sym: Symbol) -> Option<TypeId> {
    self
      .lookup_ex_symbol(sym)
      .map(|(binding, _)| binding.type_id)
  }

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

      current = scope.parent.and_then(resolve_scope);
    }

    None
  }
}
