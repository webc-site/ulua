use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{demoter::Demoter, extern_type::ExternType},
  type_aliases::type_id::TypeId,
};

impl Demoter {
  pub fn ignore_children(&mut self, ty: TypeId) -> bool {
    let et = get_type_id::<ExternType>(ty);
    !et.is_none()
  }
}
