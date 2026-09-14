use crate::{
  records::{path_builder::PathBuilder, property_type_path::Property},
  type_aliases::component::Component,
};

impl PathBuilder {
  pub fn read_prop(&mut self, name: &str) -> &mut Self {
    self
      .components
      .push(Component::Property(Property::property_string_bool(
        name, true,
      )));
    self
  }
}
