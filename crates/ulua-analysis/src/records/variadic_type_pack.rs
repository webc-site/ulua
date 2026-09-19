use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct VariadicTypePack {
  pub(crate) ty: TypeId,
  pub(crate) hidden: bool,
}

impl VariadicTypePack {
  pub fn new(ty: TypeId) -> Self {
    Self { ty, hidden: false }
  }
}

unsafe impl Send for VariadicTypePack {}
unsafe impl Sync for VariadicTypePack {}
