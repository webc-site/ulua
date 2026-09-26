use crate::{
  enums::pack_field::PackField, macros::path_builder_step, type_aliases::component::Component,
};

path_builder_step!(
  trait PathBuilderTail,
  tail() => Component::PackField(PackField::Tail),
);
