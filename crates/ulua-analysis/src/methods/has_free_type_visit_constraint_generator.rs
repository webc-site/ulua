use crate::records::has_free_type::HasFreeType;

impl HasFreeType {
  pub fn visit_type_id_extern_type(&mut self) {
    self.result = false;
  }

  pub fn visit_type_id_free_type(&mut self) {
    self.result = true;
  }

  pub fn visit_type_pack_id_free_type_pack(&mut self) {
    self.result = true;
  }
}
