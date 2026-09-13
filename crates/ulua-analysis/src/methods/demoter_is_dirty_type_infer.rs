use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{demoter::Demoter, free_type::FreeType},
  type_aliases::type_id::TypeId,
};

impl Demoter {
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    let ftv = get_type_id::<FreeType>(ty);
    !ftv.is_none()
  }
}
