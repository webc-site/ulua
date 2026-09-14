//! @interface-stub
use crate::{
  records::{free_type_pack::FreeTypePack, inplace_demoter::InplaceDemoter},
  type_aliases::type_pack_id::TypePackId,
};

impl InplaceDemoter {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_type_pack_id_free_type_pack(
    &mut self,
    tp: TypePackId,
    ftp_ref: &FreeTypePack,
  ) -> bool {
    unsafe {
      if (*tp).owning_arena != self.arena {
        return false;
      }

      let ftp = ftp_ref as *const FreeTypePack as *mut FreeTypePack;
      if (*ftp).level.subsumes_strict(&self.new_level) {
        (*ftp).level = self.new_level;
        return true;
      }
    }

    false
  }
}
