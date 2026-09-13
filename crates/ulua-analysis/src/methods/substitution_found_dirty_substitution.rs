use crate::{
  functions::follow_type::follow_type_id, records::substitution::Substitution,
  type_aliases::type_id::TypeId,
};

impl Substitution {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// C++ `Substitution::foundDirty(TypeId)` (`Substitution.cpp:724-735`).
  ///
  /// The `isDirty` / `clean` calls are virtual in C++ and dispatch into the
  /// concrete subclass; here they go through the subclass-installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable). The
  /// first `follow` is `log->follow`; the second is `Luau::follow` (the free
  /// function `follow_type_id`).
  pub unsafe fn found_dirty_type_id(&mut self, ty: TypeId) {
    let ty = unsafe { (*self.base.log).follow_type_id(ty) };

    if self.new_types.contains(&ty) {
      return;
    }

    let owner = self.base.vtable.owner;
    let is_dirty = self
      .base
      .vtable
      .is_dirty_ty
      .expect("Substitution::isDirty(TypeId) override not installed");

    let new_ty = if is_dirty(owner, ty) {
      let clean = self
        .base
        .vtable
        .clean_ty
        .expect("Substitution::clean(TypeId) override not installed");
      let cleaned = clean(owner, ty);
      follow_type_id(cleaned)
    } else {
      let cloned = self.clone_type_id(ty);
      follow_type_id(cloned)
    };

    *self.new_types.get_or_insert(ty) = new_ty;
  }
}
