use crate::records::{property_type::Property, type_stringifier::TypeStringifier};

impl TypeStringifier {
  pub fn stringify_string_property(&mut self, name: &str, prop: &Property) {
    self.type_stringifier_new_stringify(name, prop);
  }
}
