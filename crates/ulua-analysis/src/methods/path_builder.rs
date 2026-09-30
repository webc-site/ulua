//! `path_builder` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::mem::take;

use crate::{
  enums::{pack_field::PackField, type_field::TypeField, variant::Variant},
  macros::path_builder_step,
  records::{
    index::Index, pack_slice::PackSlice, path::Path, path_builder::PathBuilder,
    property_type_path::Property,
  },
  type_aliases::component::Component,
};

path_builder_step!(
  trait PathBuilderArgs,
  args() => Component::PackField(PackField::Arguments),
);

pub trait PathBuilderBuild {
  fn build(&mut self) -> Path;
}
impl PathBuilderBuild for PathBuilder {
  fn build(&mut self) -> Path {
    Path::from_components(take(&mut self.components))
  }
}

path_builder_step!(
  trait PathBuilderIndex,
  index(i: usize) => Component::Index(Index { index: i, variant: Variant::Pack }),
);

path_builder_step!(
  index_key() => Component::TypeField(TypeField::IndexLookup),
);

path_builder_step!(
  index_value() => Component::TypeField(TypeField::IndexResult),
);

path_builder_step!(
  trait PathBuilderLb,
  lb() => Component::TypeField(TypeField::LowerBound),
);

path_builder_step!(
  trait PathBuilderMt,
  mt() => Component::TypeField(TypeField::Metatable),
);

path_builder_step!(
  trait PathBuilderNegated,
  negated() => Component::TypeField(TypeField::Negated),
);

path_builder_step!(
  pack_slice(start_index: usize) => Component::PackSlice(PackSlice { start_index }),
);

path_builder_step!(
  // C++ `prop(name)` constructs `Property{name}` (default is_read = true).
  prop(name: &str) => Component::Property(Property::property_string_bool(name, true)),
);

path_builder_step!(
  read_prop(name: &str) => Component::Property(Property::property_string_bool(name, true)),
);

path_builder_step!(
  trait PathBuilderRets,
  rets() => Component::PackField(PackField::Returns),
);

path_builder_step!(
  trait PathBuilderTail,
  tail() => Component::PackField(PackField::Tail),
);

path_builder_step!(
  trait PathBuilderUb,
  ub() => Component::TypeField(TypeField::UpperBound),
);

// Source: `Analysis/src/TypePath.cpp:239-243` (hand-ported)

path_builder_step!(
  trait PathBuilderVariadic,
  variadic() => Component::TypeField(TypeField::Variadic),
);

path_builder_step!(
  write_prop(name: &str) => Component::Property(Property::property_string_bool(name, false)),
);
