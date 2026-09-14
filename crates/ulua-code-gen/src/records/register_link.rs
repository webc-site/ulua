use ulua_common::records::dense_hash_table::DenseDefault;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[derive(Default)]
pub struct RegisterLink {
  pub reg: u8,
  pub version: u32,
}

impl DenseDefault for RegisterLink {
  fn dense_default() -> Self {
    Self::default()
  }
}
