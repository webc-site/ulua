use crate::{
  records::scope::Scope,
  type_aliases::{def_id_def::DefId, type_id::TypeId},
};

impl Scope {
  pub fn lookup_unrefined_type(&self, def: DefId) -> Option<TypeId> {
    let mut current: Option<&Scope> = Some(self);
    while let Some(scope) = current {
      if let Some(ty) = scope.lvalue_types.find(&def) {
        return Some(*ty);
      }

      current = scope.parent.as_ref().map(|scoped| scoped.as_ref());
    }

    None
  }
}
