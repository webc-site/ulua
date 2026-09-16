use ulua_common::records::dense_hash_table::DenseDefault;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TableShape {
  pub(crate) array_size: u32,
  pub(crate) hash_size: u32,
}

impl DenseDefault for TableShape {
  fn dense_default() -> Self {
    Self::default()
  }
}
