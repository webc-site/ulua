use crate::{
  records::{path_builder::PathBuilder, property_type_path::Property},
  type_aliases::component::Component,
};

impl PathBuilder {
  pub fn prop(&mut self, name: &str) -> &mut Self {
    // C++ `prop(name)` constructs `Property{name}` (default is_read = true).
    self
      .components
      .push(Component::Property(Property::property_string_bool(
        name, true,
      )));
    self
  }
}
