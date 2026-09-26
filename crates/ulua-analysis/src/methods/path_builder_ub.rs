use crate::{
  enums::type_field::TypeField, macros::path_builder_step, type_aliases::component::Component,
};

path_builder_step!(
  trait PathBuilderUb,
  ub() => Component::TypeField(TypeField::UpperBound),
);
