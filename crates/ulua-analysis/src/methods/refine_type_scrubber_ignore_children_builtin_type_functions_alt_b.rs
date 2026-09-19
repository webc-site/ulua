//! C++ `RefineTypeScrubber::ignoreChildren(TypeId ty)`
//! (BuiltinTypeFunctions.cpp:1126-1129): `return !is<UnionType,
//! IntersectionType>(ty);`
use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    intersection_type::IntersectionType, refine_type_scrubber::RefineTypeScrubber,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl RefineTypeScrubber {
  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    let is_union = !get_type_id::<UnionType>(ty).is_none();
    let is_intersection = !get_type_id::<IntersectionType>(ty).is_none();
    !(is_union || is_intersection)
  }
}
