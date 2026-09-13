use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{extern_type::ExternType, union_type::UnionType, widen::Widen},
  type_aliases::type_id::TypeId,
};

impl Widen {
  pub fn widen_ignore_children(&self, ty: TypeId) -> bool {
    let et = get_type_id::<ExternType>(ty);
    if !et.is_none() {
      return true;
    }

    let ut = get_type_id::<UnionType>(ty);
    ut.is_none()
  }
}
