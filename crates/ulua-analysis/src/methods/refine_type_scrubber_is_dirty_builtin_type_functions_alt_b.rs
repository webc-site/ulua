use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    intersection_type::IntersectionType, refine_type_scrubber::RefineTypeScrubber,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl RefineTypeScrubber {
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    if let Some(ut) = get_type_id::<UnionType>(ty) {
      for &option in &ut.options {
        if option == self.needle {
          return true;
        }
      }
    } else if let Some(it) = get_type_id::<IntersectionType>(ty) {
      for &part in &it.parts {
        if part == self.needle {
          return true;
        }
      }
    }
    ty == self.needle
  }
}
