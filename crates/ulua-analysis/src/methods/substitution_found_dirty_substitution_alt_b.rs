use crate::{
  functions::follow_type_pack::follow as follow_type_pack_id, records::substitution::Substitution,
  type_aliases::type_pack_id::TypePackId,
};

impl Substitution {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// C++ `Substitution::foundDirty(TypePackId)` (`Substitution.cpp:737-748`).
  ///
  /// See [`Substitution::found_dirty_type_id`] for the dispatch/`follow`
  /// details; this is the type-pack twin.
  pub unsafe fn found_dirty_type_pack_id(&mut self, tp: TypePackId) {
    let tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

    if self.new_packs.contains(&tp) {
      return;
    }

    let owner = self.base.vtable.owner;
    let is_dirty = self
      .base
      .vtable
      .is_dirty_tp
      .expect("Substitution::isDirty(TypePackId) override not installed");

    let new_tp = if is_dirty(owner, tp) {
      let clean = self
        .base
        .vtable
        .clean_tp
        .expect("Substitution::clean(TypePackId) override not installed");
      let cleaned = clean(owner, tp);
      unsafe { follow_type_pack_id(cleaned) }
    } else {
      let cloned = self.clone_type_pack_id(tp);
      unsafe { follow_type_pack_id(cloned) }
    };

    *self.new_packs.get_or_insert(tp) = new_tp;
  }
}
