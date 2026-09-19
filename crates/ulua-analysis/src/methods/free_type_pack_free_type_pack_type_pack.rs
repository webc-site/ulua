use core::ptr::null_mut;

use crate::{
  functions::fresh_index::fresh_index,
  records::{free_type_pack::FreeTypePack, type_level::TypeLevel},
};
impl FreeTypePack {
  pub fn free_type_pack_type_level(&mut self, level: TypeLevel) {
    self.index = fresh_index();
    self.level = level;
    self.scope = null_mut();
  }
}
