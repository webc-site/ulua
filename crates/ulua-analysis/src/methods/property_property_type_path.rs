use crate::records::property_type_path::Property;

impl Property {
  pub fn property_string_bool(name: &str, read: bool) -> Self {
    Property {
      name: name.to_owned(),
      is_read: read,
    }
  }
}
