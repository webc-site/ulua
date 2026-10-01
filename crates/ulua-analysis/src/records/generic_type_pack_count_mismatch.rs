#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GenericTypePackCountMismatch {
  pub(crate) sub_ty_generic_pack_count: usize,
  pub(crate) super_ty_generic_pack_count: usize,
}

impl GenericTypePackCountMismatch {
  pub fn sub_ty_generic_pack_count(&self) -> usize {
    self.sub_ty_generic_pack_count
  }

  pub fn super_ty_generic_pack_count(&self) -> usize {
    self.super_ty_generic_pack_count
  }
}
