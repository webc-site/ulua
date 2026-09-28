use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{free_type_pack::FreeTypePack, scope::Scope, type_level::TypeLevel},
};

impl FreeTypePack {
  pub fn free_type_pack_type_level(&mut self, level: TypeLevel) {
    self.index = fresh_index();
    self.level = level;
    self.scope = null_mut();
  }

  pub fn free_type_pack_scope_polarity(&mut self, scope: *mut Scope, polarity: Polarity) {
    self.index = fresh_index();
    self.level = TypeLevel::default();
    self.scope = scope;
    self.polarity = polarity;
  }
}
