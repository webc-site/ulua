use ulua_common::FFlag;

use crate::records::normalized_type::NormalizedType;
impl NormalizedType {
  pub fn is_subtype_of_string(&self) -> bool {
    if FFlag::LuauIntegerType2.get() {
      self.has_strings()
        && !self.has_tops()
        && !self.has_booleans()
        && !self.has_extern_types()
        && !self.has_errors()
        && !self.has_nils()
        && !self.has_numbers()
        && !self.has_threads()
        && !self.has_buffers()
        && !self.has_tables()
        && !self.has_functions()
        && !self.has_tyvars()
        && !self.has_integers()
    } else {
      self.has_strings()
        && !self.has_tops()
        && !self.has_booleans()
        && !self.has_extern_types()
        && !self.has_errors()
        && !self.has_nils()
        && !self.has_numbers()
        && !self.has_threads()
        && !self.has_buffers()
        && !self.has_tables()
        && !self.has_functions()
        && !self.has_tyvars()
    }
  }
}
