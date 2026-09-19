use ulua_common::records::dense_hash_table::DenseDefault;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TableConstantKind {
  #[default]
  ConstantTable,
  ConstantOther,
  NotConstant,
}

impl DenseDefault for TableConstantKind {
  fn dense_default() -> Self {
    Self::default()
  }
}
