//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/TypePath.h:124:component`
//! Source: `Analysis/include/Luau/TypePath.h` (TypePath.h:124, hand-ported)

use crate::{
  enums::{pack_field::PackField, type_field::TypeField},
  records::{
    generic_pack_mapping::GenericPackMapping, index::Index, pack_slice::PackSlice,
    property_type_path::Property, reduction::Reduction,
  },
};

// C++: using Component = Luau::Variant<Property, Index, TypeField, PackField,
//                                      PackSlice, Reduction, GenericPackMapping>;
// 7 alternatives — dedicated enum per the Type-SCC convention.
#[derive(Debug, Clone, PartialEq)]
pub enum Component {
  Property(Property),
  Index(Index),
  TypeField(TypeField),
  PackField(PackField),
  PackSlice(PackSlice),
  Reduction(Reduction),
  GenericPackMapping(GenericPackMapping),
}
