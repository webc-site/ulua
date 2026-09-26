use crate::{
  functions::get_type,
  records::{demoter::Demoter, extern_type::ExternType},
  type_aliases::type_id::TypeId,
};

impl Demoter {
  pub fn ignore_children(&mut self, ty: TypeId) -> bool {
    let et = get_type::get::<ExternType>(ty);
    et.is_some()
  }
}
