use ulua_common::functions::hash_combine::hash_combine;

use crate::records::{
  hash_subtype_constraint_record::HashSubtypeConstraintRecord,
  subtype_constraint_record::SubtypeConstraintRecord,
};

impl HashSubtypeConstraintRecord {
  pub fn operator_call(&self, c: &SubtypeConstraintRecord) -> usize {
    let mut result: usize = 0;
    hash_combine(&mut result, c.sub_ty as usize);
    hash_combine(&mut result, c.super_ty as usize);
    hash_combine(&mut result, c.variance as usize);
    result
  }
}
