use core::convert::Infallible;

use ulua_common::records::dense_hash_table::{DenseEq, DenseHasher};

use crate::type_aliases::name_type::Name;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct HashBoolNamePair {
  pub(crate) _unused: Option<Infallible>,
}

// Wires the C++ `HashBoolNamePair` functor (TypeInfer.cpp:202, used as the
// `Hash` template parameter of `DenseHashSet<std::pair<bool, Name>, HashBoolNamePair>`)
// into the container's `DenseHasher`/`DenseEq` traits so the set is usable.
impl DenseHasher<(bool, Name)> for HashBoolNamePair {
  fn hash(&self, key: &(bool, Name)) -> usize {
    self.operator_call(key)
  }
}

impl DenseEq<(bool, Name)> for HashBoolNamePair {
  fn eq(&self, a: &(bool, Name), b: &(bool, Name)) -> bool {
    a == b
  }
}
