//! Source: `Analysis/src/TypePath.cpp:239-243` (hand-ported)
use crate::{
  enums::type_field::TypeField, macros::path_builder_step, type_aliases::component::Component,
};

path_builder_step!(
  trait PathBuilderVariadic,
  variadic() => Component::TypeField(TypeField::Variadic),
);
