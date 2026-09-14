use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
  pub fn has_tables(&self) -> bool {
    !self.tables.is_never()
  }
}
