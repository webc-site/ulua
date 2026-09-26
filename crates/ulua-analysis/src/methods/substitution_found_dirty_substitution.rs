use crate::{
  functions::{follow_type, follow_type_pack},
  records::substitution::Substitution,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Substitution {
  /// # Safety
  /// `ty` 须为指向存活类型 arena 节点的有效 `TypeId`，且 `self.base.log` 须为
  /// 非空并指向存活的 `TxnLog`——首行 `(*self.base.log).follow_type_id(ty)` 会
  /// 解引用二者。`self.base.vtable` 的 `owner` 及 `is_dirty_ty`/`clean_ty` 覆写
  /// 须已安装（否则下方 `expect` panic）。
  /// C++ `Substitution::foundDirty(TypeId)` (`cpp/Analysis/src/Substitution.cpp:723`).
  ///
  /// The `isDirty` / `clean` calls are virtual in C++ and dispatch into the
  /// concrete subclass; here they go through the subclass-installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable). The
  /// first `follow` is `log->follow`; the second is `Luau::follow` (the free
  /// function `follow_type_id`).
  pub unsafe fn found_dirty_type_id(&mut self, ty: TypeId) {
    // Safety: `self.base.log` 为构造 Substitution 时接线的非空 `*const TxnLog`（进程级
    // `TxnLog::empty()` 单例或调用方传入的活 log），比本 Substitution 长寿；`follow_type_id`
    // 为 `&self` 只读，`ty` 是存活类型句柄。单线程求解循环中重建共享引用无并发别名。
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
      follow_type::follow(cleaned)
    } else {
      let cloned = self.clone_type_id(ty);
      follow_type::follow(cloned)
    };

    *self.new_types.get_or_insert(ty) = new_ty;
  }

  /// # Safety
  /// `tp` 须为指向存活类型 arena 节点的有效 `TypePackId`，且 `self.base.log` 非空并指向存活
  /// 的 `TxnLog`（首行 follow 会解引用二者）；`self.base.vtable` 的 `owner` 与 dirty/clean 覆写
  /// 须已安装。单线程、无并发写别名。
  /// C++ `Substitution::foundDirty(TypePackId)` (`cpp/Analysis/src/Substitution.cpp:736`).
  ///
  /// See [`Substitution::found_dirty_type_id`] for the dispatch/`follow`
  /// details; this is the type-pack twin.
  pub unsafe fn found_dirty_type_pack_id(&mut self, tp: TypePackId) {
    // Safety: 同类型孪生——`self.base.log` 为非空存活的 `*const TxnLog`，`follow_type_pack_id`
    // 为 `&self` 只读，`tp` 是存活类型包句柄；单线程求解中无并发别名。
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
      follow_type_pack::follow(cleaned)
    } else {
      let cloned = self.clone_type_pack_id(tp);
      follow_type_pack::follow(cloned)
    };

    *self.new_packs.get_or_insert(tp) = new_tp;
  }
}
