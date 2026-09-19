use ulua_common::functions::hash_combine::hash_combine;

use crate::records::{
  hash_subtype_constraint_record::HashSubtypeConstraintRecord,
  subtype_constraint_record::SubtypeConstraintRecord,
};

impl HashSubtypeConstraintRecord {
  pub fn operator_call(&self, c: &SubtypeConstraintRecord) -> usize {
    [c.sub_ty as usize, c.super_ty as usize, c.variance as usize]
      .into_iter()
      .fold(0, hash_combine)
  }
}
