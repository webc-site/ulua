use crate::records::cannot_assign_to_never::CannotAssignToNever;

impl CannotAssignToNever {
  #[inline]
  pub fn operator_eq(&self, rhs: &CannotAssignToNever) -> bool {
    self.cause == rhs.cause && self.rhs_type == rhs.rhs_type && self.reason == rhs.reason
  }
}

#[cfg(test)]
mod tests {
  use alloc::vec;

  use super::CannotAssignToNever;
  use crate::{enums::reason::Reason, records::r#type::Type, type_aliases::type_id::TypeId};

  /// TypeId 是 `*const Type`，测试用悬垂指针充当不同的类型标识。
  fn ty(v: u8) -> TypeId {
    v as *const Type
  }

  #[test]
  fn equal_values() {
    assert!(
      CannotAssignToNever::new(ty(1), vec![ty(2)], Reason::PropertyNarrowed).operator_eq(
        &CannotAssignToNever::new(ty(1), vec![ty(2)], Reason::PropertyNarrowed)
      )
    );
  }

  #[test]
  fn different_cause_not_equal() {
    assert!(
      !CannotAssignToNever::new(ty(1), vec![ty(2)], Reason::PropertyNarrowed).operator_eq(
        &CannotAssignToNever::new(ty(1), vec![ty(3)], Reason::PropertyNarrowed)
      )
    );
    assert!(
      !CannotAssignToNever::new(ty(1), vec![ty(2)], Reason::PropertyNarrowed).operator_eq(
        &CannotAssignToNever::new(ty(1), vec![], Reason::PropertyNarrowed)
      )
    );
  }

  #[test]
  fn different_rhs_type_or_reason_not_equal() {
    assert!(
      !CannotAssignToNever::new(ty(1), vec![ty(2)], Reason::PropertyNarrowed).operator_eq(
        &CannotAssignToNever::new(ty(4), vec![ty(2)], Reason::PropertyNarrowed)
      )
    );
  }
}
