use crate::records::has_free_type::HasFreeType;

impl HasFreeType {
  pub fn visit_type_id_free_type(&mut self) {
    self.result = true;
  }
}
