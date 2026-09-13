use crate::records::property_type_path::Property;

impl Property {
  pub fn read(&mut self, name: &str) -> Property {
    Property::property_string_bool(name, true)
  }
}
