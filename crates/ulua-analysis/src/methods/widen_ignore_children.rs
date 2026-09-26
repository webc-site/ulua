use crate::{
  functions::get_type,
  records::{extern_type::ExternType, union_type::UnionType, widen::Widen},
  type_aliases::type_id::TypeId,
};

impl Widen {
  pub fn widen_ignore_children(&self, ty: TypeId) -> bool {
    let et = get_type::get::<ExternType>(ty);
    if et.is_some() {
      return true;
    }

    let ut = get_type::get::<UnionType>(ty);
    ut.is_none()
  }
}
