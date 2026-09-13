use crate::{records::widen::Widen, type_aliases::type_pack_id::TypePackId};

impl Widen {
  pub fn operator_call(&mut self, tp: TypePackId) -> TypePackId {
    self.install_substitution_vtable();
    self.base.substitute_type_pack_id(tp).unwrap_or(tp)
  }
}
