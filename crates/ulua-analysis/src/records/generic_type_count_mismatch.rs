#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GenericTypeCountMismatch {
  pub(crate) sub_ty_generic_count: usize,
  pub(crate) super_ty_generic_count: usize,
}

impl GenericTypeCountMismatch {
  pub fn sub_ty_generic_count(&self) -> usize {
    self.sub_ty_generic_count
  }

  pub fn super_ty_generic_count(&self) -> usize {
    self.super_ty_generic_count
  }
}
