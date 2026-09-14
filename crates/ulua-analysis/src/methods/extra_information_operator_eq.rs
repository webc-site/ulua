use crate::records::extra_information::ExtraInformation;

impl ExtraInformation {
  #[inline]
  pub fn operator_eq(&self, rhs: &ExtraInformation) -> bool {
    self.message == rhs.message
  }
}
