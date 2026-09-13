use core::ptr::null_mut;

use crate::{
  functions::{
    follow_type_pack::follow_type_pack_id, get_mutable_type_pack::get_mutable_type_pack_id,
  },
  records::{
    free_type_pack::FreeTypePack, generic_type_pack::GenericTypePack, type_cloner::TypeCloner,
    type_pack_var::TypePackVar,
  },
  type_aliases::{type_or_pack::TypeOrPack, type_pack_id::TypePackId},
};
impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn shallow_clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let tp = unsafe { follow_type_pack_id(tp) };

    if let Some(clone) = self.find_type_pack_id(tp) {
      return clone;
    } else if unsafe { (*tp).persistent } && tp != self.force_tp {
      return tp;
    }

    let target = unsafe {
      (*self.arena).add_type_pack_type_pack_var(TypePackVar {
        ty: (*tp).ty.clone(),
        persistent: false,
        owning_arena: null_mut(),
      })
    };

    // null for ordinary clones (Clone.cpp:194-197), fresh scope for the
    // fragment cloner override (Clone.cpp:531-534). Generic packs always null.
    if let Some(generic) = get_mutable_type_pack_id::<GenericTypePack>(target) {
      generic.scope = null_mut();
    } else if let Some(free) = get_mutable_type_pack_id::<FreeTypePack>(target) {
      free.scope = self.replacement_for_null_scope;
    }

    unsafe { (*self.packs).insert(tp, target) };
    self.queue.push(TypeOrPack::V1(target));
    target
  }
}
