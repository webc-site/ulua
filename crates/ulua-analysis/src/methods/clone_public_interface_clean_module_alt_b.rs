use core::ptr::null_mut;

use crate::{
  functions::{get_mutable_type_pack::get_mutable_type_pack_id, get_type_pack::get_type_pack_id},
  records::{
    blocked_type_pack::BlockedTypePack, clone_public_interface::ClonePublicInterface,
    free_type_pack::FreeTypePack, generic_type_pack::GenericTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl ClonePublicInterface {
  /// `TypePackId ClonePublicInterface::clean(TypePackId tp)`.
  /// Reference: `Module.cpp:210-229`.
  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    if self.is_new_solver() {
      if get_type_pack_id::<FreeTypePack>(tp).is_some()
        || get_type_pack_id::<BlockedTypePack>(tp).is_some()
      {
        self.internal_type_escaped = true;
        // SAFETY: builtin_types 指向全局 BuiltinTypes，存活期覆盖整个分析过程。
        return unsafe { (*self.builtin_types).error_type_pack };
      }

      let cloned_tp = self.base.clone_type_pack_id(tp);
      if let Some(gtp) = get_mutable_type_pack_id::<GenericTypePack>(cloned_tp) {
        gtp.scope = null_mut();
      }
      cloned_tp
    } else {
      self.base.clone_type_pack_id(tp)
    }
  }
}
