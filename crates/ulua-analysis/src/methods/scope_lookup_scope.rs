use crate::{
  records::{scope::Scope, symbol::Symbol},
  type_aliases::type_id::TypeId,
};

impl Scope {
  pub fn lookup_symbol(&self, sym: Symbol) -> Option<TypeId> {
    self
      .lookup_ex_symbol(sym)
      .map(|(binding, _)| binding.type_id)
  }
}
