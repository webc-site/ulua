use crate::{
  functions::shallow_clone_substitution::shallow_clone_type_id_type_arena_txn_log,
  records::substitution::Substitution, type_aliases::type_id::TypeId,
};

impl Substitution {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn clone_type_id(&mut self, ty: TypeId) -> TypeId {
    let arena = unsafe { &mut *self.arena };
    unsafe { shallow_clone_type_id_type_arena_txn_log(ty, arena, self.base.log) }
  }
}
