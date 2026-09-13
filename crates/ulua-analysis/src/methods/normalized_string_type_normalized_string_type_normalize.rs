use alloc::collections::BTreeMap;

use crate::records::normalized_string_type::NormalizedStringType;

impl NormalizedStringType {
  pub fn normalized_string_type(&mut self) {
    self.normalized_string_type_bool_map_string_type_id(false, BTreeMap::new());
  }
}
