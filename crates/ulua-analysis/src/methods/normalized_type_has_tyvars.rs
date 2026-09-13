use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
  pub fn has_tyvars(&self) -> bool {
    !self.tyvars.is_empty()
  }
}
