use ulua_common::FFlag;

use crate::records::normalized_type::NormalizedType;
impl NormalizedType {
  pub fn is_nil(&self) -> bool {
    if !self.has_nils() {
      return false;
    }

    let mut result = !self.has_tops()
      && !self.has_booleans()
      && !self.has_extern_types()
      && !self.has_numbers()
      && !self.has_strings()
      && !self.has_threads()
      && !self.has_buffers()
      && !self.has_tables()
      && !self.has_functions()
      && !self.has_tyvars();

    if FFlag::LuauIntegerType2.get() {
      result = result && !self.has_integers();
    }

    result
  }
}
