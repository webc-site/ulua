use crate::records::property_type_path::Property;

impl Property {
  pub fn write(name: &str) -> Self {
    Property::property_string_bool(name, false)
  }
}
