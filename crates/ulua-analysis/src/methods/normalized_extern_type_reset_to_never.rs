use crate::records::normalized_extern_type::NormalizedExternType;

impl NormalizedExternType {
  pub fn reset_to_never(&mut self) {
    self.ordering.clear();
    self.extern_types.clear();
    self.shape_extensions.clear();
  }
}
