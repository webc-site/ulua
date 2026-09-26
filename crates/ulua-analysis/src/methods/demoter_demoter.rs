//! `Demoter` 构造 + 虚函数挂接。
//!
//! C++ `Demoter(TypeArena* arena, NotNull<BuiltinTypes> builtins)`（TypeInfer.cpp:780）。
//! `isDirty` / `clean` / `ignoreChildren` 的覆写体在各自的
//! `demoter_*` 方法文件中，此处经 `SubstitutionVtable` 装上——与
//! `Replacer::install_substitution_vtable` 同款模式。

use crate::{
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, demoter::Demoter,
    substitution::Substitution, tarjan::SubstitutionVtable, txn_log::TxnLog, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn demoter_is_dirty_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 由 install_substitution_vtable 以 `self as *mut Demoter as *mut ()`
  // 捕获，vtable 嵌在 Demoter.base.base 内，故 Demoter 比任何一次分派长寿——回投
  // `*mut Demoter` 恒指向存活且对齐的 Demoter。is_dirty_type_id 只经全局 get_type_id
  // 读 arena（独立分配），不触碰持有者的可变借用；单线程串行分派无别名。
  unsafe { (*(owner as *mut Demoter)).is_dirty_type_id(ty) }
}

fn demoter_is_dirty_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: 同上——owner 恒指向存活且对齐的 Demoter（vtable 内嵌于该对象）；只经
  // get_type_pack_id 读 arena，单线程分派下无并发可变借用。
  unsafe { (*(owner as *mut Demoter)).is_dirty_type_pack_id(tp) }
}

fn demoter_clean_ty(owner: *mut (), ty: TypeId) -> TypeId {
  // Safety: owner 恒指向存活 Demoter；clean_type_id 仅向 self.arena（独立堆分配）
  // 追加节点。分派方 Tarjan::visit_scc 在调用前已把 owner 与函数指针拷出，调用期
  // 不再持有对持有者的借用，故重建的可变访问与遍历无重叠别名（单线程串行）。
  unsafe { (*(owner as *mut Demoter)).clean_type_id(ty) }
}

fn demoter_clean_tp(owner: *mut (), tp: TypePackId) -> TypePackId {
  // Safety: 同 clean_ty——owner 指向存活 Demoter，写目标为 arena 独立分配；
  // 单线程、无跨调用的借用持有，重建可变访问不与其他别名冲突。
  unsafe { (*(owner as *mut Demoter)).clean_type_pack_id(tp) }
}

fn demoter_found_dirty_ty(owner: *mut (), ty: TypeId) {
  // Safety: owner 指向存活 Demoter。found_dirty 经 `base` 取 Substitution 子对象，
  // 仅写 Substitution.new_types；而分派方 visit_scc 只读写 Tarjan 自身字段
  // (nodes/stack/dirty) 且拷出指针后不再借用持有者——两访问命中的子对象字段互不
  // 重叠，单线程串行下不构成并发别名。
  unsafe { (*(owner as *mut Demoter)).base.found_dirty_type_id(ty) }
}

fn demoter_found_dirty_tp(owner: *mut (), tp: TypePackId) {
  // Safety: 同 found_dirty_ty——owner 指向存活 Demoter；写 Substitution.new_packs
  // 与分派方读写的 Tarjan 字段不相交，单线程无别名。
  unsafe { (*(owner as *mut Demoter)).base.found_dirty_type_pack_id(tp) }
}

fn demoter_ignore_children_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 指向存活 Demoter；ignore_children 仅经 get_type_id 只读 arena 判定
  // 变体，不改动持有者状态，单线程分派无别名。
  unsafe { (*(owner as *mut Demoter)).ignore_children(ty) }
}

// C++ 未覆写 `ignoreChildren(TypePackId)`，保持基类默认 false。
fn demoter_ignore_children_tp(_owner: *mut (), _tp: TypePackId) -> bool {
  false
}

impl Demoter {
  pub fn new(arena: Handle<TypeArena>, builtins: Handle<BuiltinTypes>) -> Self {
    let mut this = Demoter {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      arena,
      builtins,
    };
    this.install_substitution_vtable();
    this
  }

  pub(crate) fn install_substitution_vtable(&mut self) {
    let owner = self as *mut Demoter as *mut ();
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(demoter_is_dirty_ty),
      is_dirty_tp: Some(demoter_is_dirty_tp),
      clean_ty: Some(demoter_clean_ty),
      clean_tp: Some(demoter_clean_tp),
      found_dirty_ty: Some(demoter_found_dirty_ty),
      found_dirty_tp: Some(demoter_found_dirty_tp),
      ignore_children_ty: Some(demoter_ignore_children_ty),
      ignore_children_tp: Some(demoter_ignore_children_tp),
      ignore_children_visit_ty: Some(demoter_ignore_children_ty),
      ignore_children_visit_tp: Some(demoter_ignore_children_tp),
    };
  }
}
