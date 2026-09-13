use ulua_common::records::dense_hash_table::DenseDefault;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Mark {
  #[default]
  None,
  Temporary,
  Permanent,
}

// Required so `Mark` can be a `DenseHashMap` value type in `Frontend::parseGraph`
// (C++: `DenseHashMap<SourceNode*, Mark> seen(nullptr)`). The map default-inserts
// `Mark{}` for absent keys, which is the zero-initialized first enumerator `None`.
impl DenseDefault for Mark {
  fn dense_default() -> Self {
    Self::None
  }
}
