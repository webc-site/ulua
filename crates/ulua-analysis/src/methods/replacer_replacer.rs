use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  macros::substitution_entry::substitution_entry,
  records::{
    arena_handle::Handle, replacer::Replacer, substitution::Substitution,
    tarjan::SubstitutionVtable, txn_log::TxnLog, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn replacer_is_dirty_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: `owner` 只可能由本模块 install_substitution_vtable 写入（`self as
  // *mut Replacer as *mut ()`），回转类型即真实类型；thunk 仅在宿主自己的
  // substitute_type_id/substitute_type_pack_id 遍历中被回调，遍历由 `&mut self`
  // 驱动，宿主存活且独占。`(*self.replacements)` 是 Replacer::new 收下的调用方
  // DenseHashMap 裸指针（C++ const& 成员的 NotNull 语义），调用方保证其比本次
  // substitute 调用长寿，find 为只读查询。
  unsafe { (*(owner as *mut Replacer)).is_dirty_type_id(ty) }
}

fn replacer_is_dirty_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: owner 回转前提同 ty 版（唯一装配点、宿主经 &mut self 驱动遍历）；
  // replacement_packs 亦为构造期接线的调用方 DenseHashMap，find 只读、tp 按值取哈希。
  unsafe { (*(owner as *mut Replacer)).is_dirty_type_pack_id(tp) }
}

fn replacer_clean_ty(owner: *mut (), ty: TypeId) -> TypeId {
  // Safety: owner 回转同装配点不变量。clean_type_id 的前置条件（replacements 非空
  // 存活、ty 是替换表中存在的 dirty 句柄）由 is_dirty_ty 刚回调成功的同一表保证；
  // dont_traverse_into_type_id 只写宿主自有的 no_traverse 集合（与 `&mut self` 同半径）。
  unsafe { (*(owner as *mut Replacer)).clean_type_id(ty) }
}

fn replacer_clean_tp(owner: *mut (), tp: TypePackId) -> TypePackId {
  // Safety: pack 侧孪生分支——同一装配不变量；替换值从存活 replacement_packs 表
  // 读出后即按 Copy 句柄返回，宿主遍历期 no_traverse 集合写入同样独占。
  unsafe { (*(owner as *mut Replacer)).clean_type_pack_id(tp) }
}

fn replacer_found_dirty_ty(owner: *mut (), ty: TypeId) {
  // Safety: 该覆写未被 Replacer 覆盖，先回转 owner 再走基类 `Substitution`（`.base`
  // 是首字段，地址偏移一致）。found_dirty_type_id 要求 `base.base.log` 非空指向存活
  // TxnLog——由 substitution_new(TxnLog::empty(), arena) 注入的进程级单例（OnceLock
  // 静态存活）；vtable 已装配正是 install_substitution_vtable 在 substitute 前刚做的事。
  unsafe { (*(owner as *mut Replacer)).base.found_dirty_type_id(ty) }
}

fn replacer_found_dirty_tp(owner: *mut (), tp: TypePackId) {
  // Safety: pack 侧孪生分支——同样的 owner 装配不变量与同样的 TxnLog::empty() 单例
  // 前置条件（基类 traverse 期间由宿主独占驱动，无并发访问该对象）。
  unsafe {
    (*(owner as *mut Replacer))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn replacer_ignore_children_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 回转合法（唯一装配点）。ignore_children 只经 get_type_id 安全门面
  // 读 ty 指向的 arena 节点判别 Extern/Function 变体（ty 为 Tarjan 遍历快照句柄，
  // 始终存活），不触碰 replacements/replacement_packs 表，也不写任何状态。
  unsafe { (*(owner as *mut Replacer)).ignore_children(ty) }
}

fn replacer_ignore_children_tp(_owner: *mut (), _tp: TypePackId) -> bool {
  false
}

impl Replacer {
  pub fn new(
    arena: Handle<TypeArena>,
    replacements: *mut DenseHashMap<TypeId, TypeId>,
    replacement_packs: *mut DenseHashMap<TypePackId, TypePackId>,
  ) -> Self {
    let this = Replacer {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      replacements,
      replacement_packs,
    };
    LUAU_ASSERT!(this.check_replacement_keys());
    this
  }

  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut Replacer as *mut ();
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(replacer_is_dirty_ty),
      is_dirty_tp: Some(replacer_is_dirty_tp),
      clean_ty: Some(replacer_clean_ty),
      clean_tp: Some(replacer_clean_tp),
      found_dirty_ty: Some(replacer_found_dirty_ty),
      found_dirty_tp: Some(replacer_found_dirty_tp),
      ignore_children_ty: Some(replacer_ignore_children_ty),
      ignore_children_tp: Some(replacer_ignore_children_tp),
      ignore_children_visit_ty: Some(replacer_ignore_children_ty),
      ignore_children_visit_tp: Some(replacer_ignore_children_tp),
    };
  }

  substitution_entry!(id, pack);
}
