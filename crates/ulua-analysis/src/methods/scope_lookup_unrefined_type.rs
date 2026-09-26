use crate::{
  records::{scope::Scope, scope_registry::resolve_scope},
  type_aliases::{def_id_def::DefId, type_id::TypeId},
};

impl Scope {
  pub fn lookup_unrefined_type(&self, def: DefId) -> Option<TypeId> {
    let mut current: Option<&Scope> = Some(self);
    while let Some(scope) = current {
      if let Some(ty) = scope.lvalue_types.find(&def) {
        return Some(*ty);
      }

      current = scope.parent.and_then(resolve_scope);
    }

    None
  }
}
