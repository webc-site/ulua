use crate::records::has_free_type::HasFreeType;

impl HasFreeType {
  pub fn visit_type_pack_id_free_type_pack(&mut self) {
    self.result = true;
  }
}
