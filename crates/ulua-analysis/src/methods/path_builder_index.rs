use crate::{
  enums::variant::Variant, macros::path_builder_step, records::index::Index,
  type_aliases::component::Component,
};

path_builder_step!(
  trait PathBuilderIndex,
  index(i: usize) => Component::Index(Index { index: i, variant: Variant::Pack }),
);
