//! @interface-stub
use alloc::sync::Arc;
use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{
    free_type_pack::FreeTypePack, module::Module, type_checker::TypeChecker, type_level::TypeLevel,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl TypeChecker {
  pub fn fresh_type_pack_type_level(&mut self, level: TypeLevel) -> TypePackId {
    unsafe {
      let module = Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
      (*module).internal_types.add_type_pack_t(FreeTypePack {
        index: fresh_index(),
        level,
        scope: null_mut(),
        polarity: Polarity::None,
      })
    }
  }
}
