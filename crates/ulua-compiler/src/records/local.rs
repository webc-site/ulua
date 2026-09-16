use ulua_common::records::dense_hash_table::DenseDefault;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Local {
  pub(crate) reg: u8,
  pub(crate) allocated: bool,
  pub(crate) captured: bool,
  pub(crate) debugpc: u32,
  pub(crate) allocpc: u32,
}

impl DenseDefault for Local {
  fn dense_default() -> Self {
    Self::default()
  }
}
