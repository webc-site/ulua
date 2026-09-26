use crate::{
  functions::{get_type, get_type_pack},
  records::{demoter::Demoter, free_type::FreeType, free_type_pack::FreeTypePack},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Demoter {
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    let ftv = get_type::get::<FreeType>(ty);
    ftv.is_some()
  }

  pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let ftp = get_type_pack::get::<FreeTypePack>(tp);
    ftp.is_some()
  }
}
