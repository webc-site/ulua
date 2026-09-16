use alloc::{collections::BTreeMap, string::String};

use crate::{records::normalized_string_type::NormalizedStringType, type_aliases::type_id::TypeId};
impl NormalizedStringType {
  pub fn normalized_string_type_bool_map_string_type_id(
    &mut self,
    is_cofinite: bool,
    singletons: BTreeMap<String, TypeId>,
  ) {
    self.is_cofinite = is_cofinite;
    self.singletons = singletons;
  }
}
