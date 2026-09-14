use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
  pub fn has_extern_types(&self) -> bool {
    !self.extern_types.is_never()
  }
}
