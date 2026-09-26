use crate::{
  functions::{begin_type::begin_intersection_type, follow_type, get_type},
  records::{
    intersection_builder::IntersectionBuilder, intersection_type::IntersectionType,
    never_type::NeverType, unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl IntersectionBuilder {
  pub fn add(&mut self, ty: TypeId) {
    let ty = follow_type::follow(ty);

    if get_type::get::<NeverType>(ty).is_some() {
      self.is_bottom = true;
      return;
    }

    if get_type::get::<UnknownType>(ty).is_some() {
      return;
    }

    if let Some(itv) = get_type::get::<IntersectionType>(ty) {
      // C++ `for (auto part : itv)` — IntersectionTypeIterator 防环展平并 follow
      // Bound,裸遍历 parts 会漏掉嵌套 intersection。
      for part in begin_intersection_type(itv) {
        self.parts.insert_type_id(part);
      }
    } else {
      self.parts.insert_type_id(ty);
    }
  }
}
