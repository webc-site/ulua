use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    never_type::NeverType, union_builder::UnionBuilder, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl UnionBuilder {
  pub fn add(&mut self, ty: TypeId) {
    let ty = follow_type_id(ty);

    if get_type_id::<NeverType>(ty).is_some() || self.is_top {
      return;
    }

    if get_type_id::<UnknownType>(ty).is_some() {
      self.is_top = true;
      return;
    }

    if let Some(utv) = get_type_id::<UnionType>(ty) {
      for &option in &utv.options {
        self.options.insert_type_id(option);
      }
    } else {
      self.options.insert_type_id(ty);
    }
  }
}
