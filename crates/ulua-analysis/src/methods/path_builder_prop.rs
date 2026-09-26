use crate::{
  macros::path_builder_step, records::property_type_path::Property,
  type_aliases::component::Component,
};

path_builder_step!(
  // C++ `prop(name)` constructs `Property{name}` (default is_read = true).
  prop(name: &str) => Component::Property(Property::property_string_bool(name, true)),
);
