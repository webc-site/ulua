use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    intersection_builder::IntersectionBuilder, intersection_type::IntersectionType,
    never_type::NeverType, unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl IntersectionBuilder {
  pub fn add(&mut self, ty: TypeId) {
    let ty = follow_type_id(ty);

    if get_type_id::<NeverType>(ty).is_some() {
      self.is_bottom = true;
      return;
    }

    if get_type_id::<UnknownType>(ty).is_some() {
      return;
    }

    if let Some(itv) = get_type_id::<IntersectionType>(ty) {
      for &part in itv.parts.iter() {
        self.parts.insert_type_id(part);
      }
    } else {
      self.parts.insert_type_id(ty);
    }
  }
}
