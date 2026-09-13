use crate::records::normalized_extern_type::NormalizedExternType;

impl NormalizedExternType {
  pub fn is_never(&self) -> bool {
    self.extern_types.is_empty()
  }
}
