use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type},
  records::{
    never_type::NeverType, union_builder::UnionBuilder, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl UnionBuilder {
  pub fn add(&mut self, ty: TypeId) {
    let ty = follow_type::follow(ty);

    if get_type::get::<NeverType>(ty).is_some() || self.is_top {
      return;
    }

    if get_type::get::<UnknownType>(ty).is_some() {
      self.is_top = true;
      return;
    }

    if let Some(utv) = get_type::get::<UnionType>(ty) {
      // C++ `for (auto option : utv)`——UnionTypeIterator 展平嵌套 union 并
      // follow,裸遍历 options 会漏掉嵌套成员。
      for option in begin_union_type(utv) {
        self.options.insert_type_id(option);
      }
    } else {
      self.options.insert_type_id(ty);
    }
  }
}
