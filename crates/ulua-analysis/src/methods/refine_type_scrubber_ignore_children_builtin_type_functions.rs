use crate::{
  functions::get_type,
  records::{
    intersection_type::IntersectionType, refine_type_scrubber::RefineTypeScrubber,
    union_type::UnionType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl RefineTypeScrubber {
  pub fn ignore_children_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    false
  }

  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    let is_union = get_type::get::<UnionType>(ty).is_some();
    let is_intersection = get_type::get::<IntersectionType>(ty).is_some();
    !(is_union || is_intersection)
  }
}
