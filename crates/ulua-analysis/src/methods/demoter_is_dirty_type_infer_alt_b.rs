use crate::{
  functions::get_type_pack::get_type_pack_id,
  records::{demoter::Demoter, free_type_pack::FreeTypePack},
  type_aliases::type_pack_id::TypePackId,
};

impl Demoter {
  pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let ftp = get_type_pack_id::<FreeTypePack>(tp);
    !ftp.is_none()
  }
}
