use crate::{
  macros::path_builder_step, records::property_type_path::Property,
  type_aliases::component::Component,
};

path_builder_step!(
  write_prop(name: &str) => Component::Property(Property::property_string_bool(name, false)),
);
