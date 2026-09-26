use crate::{
  functions::get_type,
  records::{
    intersection_type::IntersectionType, refine_type_scrubber::RefineTypeScrubber,
    union_type::UnionType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl RefineTypeScrubber {
  pub fn is_dirty_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    false
  }

  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    if let Some(ut) = get_type::get::<UnionType>(ty) {
      for &option in &ut.options {
        if option == self.needle {
          return true;
        }
      }
    } else if let Some(it) = get_type::get::<IntersectionType>(ty) {
      for &part in &it.parts {
        if part == self.needle {
          return true;
        }
      }
    }
    ty == self.needle
  }
}
