use crate::records::occurs_check_failed::OccursCheckFailed;

impl OccursCheckFailed {
  #[inline]
  pub fn operator_eq(&self, _other: &OccursCheckFailed) -> bool {
    true
  }
}
