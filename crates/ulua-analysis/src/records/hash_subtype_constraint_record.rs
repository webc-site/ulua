use core::convert::Infallible;

use ulua_common::{functions::hash_combine::hash_combine, records::dense_hash_table::DenseHasher};

use crate::records::subtype_constraint_record::SubtypeConstraintRecord;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct HashSubtypeConstraintRecord {
  pub(crate) _unused: Option<Infallible>,
}

impl DenseHasher<SubtypeConstraintRecord> for HashSubtypeConstraintRecord {
  fn hash(&self, c: &SubtypeConstraintRecord) -> usize {
    [c.sub_ty as usize, c.super_ty as usize, c.variance as usize]
      .into_iter()
      .fold(0, hash_combine)
  }
}
