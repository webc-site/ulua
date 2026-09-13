use alloc::string::ToString;

use ulua_analysis::{records::property_type::Property, type_aliases::props_type::Props};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn props(entries: Vec<(&str, Property)>) -> Props {
    entries
      .into_iter()
      .map(|(name, property)| (name.to_string(), property))
      .collect()
  }
}
