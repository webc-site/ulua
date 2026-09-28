//! `inplace_demoter` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::{
  functions::get_mutable_level::get_mutable_level,
  records::{free_type_pack::FreeTypePack, inplace_demoter::InplaceDemoter},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl InplaceDemoter {
  pub fn demote(&mut self, ty: TypeId) -> bool {
    let level = { get_mutable_level(ty) };
    if !level.is_null() && unsafe { (*level).subsumes_strict(&self.new_level) } {
      unsafe {
        *level = self.new_level;
      }
      return true;
    }
    false
  }
}

impl InplaceDemoter {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_type_pack_id_free_type_pack(
    &mut self,
    tp: TypePackId,
    ftp_ref: &FreeTypePack,
  ) -> bool {
    unsafe {
      if (*tp).owning_arena != self.arena.get().arena_id {
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
