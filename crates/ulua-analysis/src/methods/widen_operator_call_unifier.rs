use crate::{records::widen::Widen, type_aliases::type_id::TypeId};

impl Widen {
  pub fn operator_call_mut(&mut self, ty: TypeId) -> TypeId {
    self.install_substitution_vtable();
    self.base.substitute_type_id(ty).unwrap_or(ty)
  }
}
