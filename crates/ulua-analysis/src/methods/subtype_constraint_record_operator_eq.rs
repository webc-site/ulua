use crate::records::subtype_constraint_record::SubtypeConstraintRecord;

impl SubtypeConstraintRecord {
  pub fn operator_eq(&self, other: &SubtypeConstraintRecord) -> bool {
    self.sub_ty == other.sub_ty
      && self.super_ty == other.super_ty
      && self.variance == other.variance
  }
}
