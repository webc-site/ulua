use crate::{
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, substitution::Substitution,
    tarjan::SubstitutionVtable, txn_log::TxnLog, type_arena::TypeArena, widen::Widen,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn widen_is_dirty_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: `owner` 只可能由本模块 install_substitution_vtable 写入（`self as
  // *mut Widen as *mut ()`），回转类型即真实类型；该 thunk 仅在 vtable 宿主
  // 自己的 substitute 遍历中被回调，遍历全程由 `&mut self` 驱动，宿主存活且独占。
  // is_dirty_type_id 只经 `(*base.base.log)` 读 TxnLog——log 为 substitution_new
  // 注入的 TxnLog::empty() 进程级单例（OnceLock 静态存活，构造期 LUAU_ASSERT 非空）。
  unsafe { (*(owner as *mut Widen)).is_dirty_type_id(ty) }
}

fn widen_is_dirty_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: owner 回转前提同 ty 版（本模块唯一装配点，宿主经 &mut self 驱动遍历）；
  // is_dirty_type_pack_id 是无状态的常量 false 实现，不触碰任何指针字段。
  unsafe { (*(owner as *mut Widen)).is_dirty_type_pack_id(tp) }
}

fn widen_clean_ty(owner: *mut (), ty: TypeId) -> TypeId {
  // Safety: owner 回转合法性同装配点不变量。clean_type_id 先经 `(*self.base.base.log)`
  // 查 TxnLog（empty 单例，同上），再经 `self.builtin_types.get()` 读 builtin 句柄——
  // builtin_types 由 widen_widen 构造时从 Unifier 的会话句柄接线，非空、比 Widen 长寿、
  // 类型检查期不再写入；ty 为 Tarjan 遍历快照给出的 arena 存活节点句柄。
  unsafe { (*(owner as *mut Widen)).clean_type_id(ty) }
}

fn widen_clean_tp(owner: *mut (), tp: TypePackId) -> TypePackId {
  // Safety: owner 回转同 ty 版（唯一装配点给出的宿主存活与类型一致性）。该方法在
  // Widen 上是 unreachable panic 分支——is_dirty_type_pack_id 恒为 false，Tarjan
  // 只会对 dirty pack 调 clean；即便进入也只是按值转发 tp 句柄，不解引用它。
  unsafe { (*(owner as *mut Widen)).clean_type_pack_id(tp) }
}

fn widen_found_dirty_ty(owner: *mut (), ty: TypeId) {
  // Safety: 该覆写未被 Widen 覆盖，先回转 owner 再走基类 `Substitution`（`.base` 是
  // 首字段，地址偏移一致）。found_dirty_type_id 要求 `base.base.log` 非空指向存活
  // TxnLog、vtable 已装配——前者由 substitution_new(TxnLog::empty(), arena) 给出，
  // 后者正是 install_substitution_vtable 在 substitute 之前刚完成的事。
  unsafe { (*(owner as *mut Widen)).base.found_dirty_type_id(ty) }
}

fn widen_found_dirty_tp(owner: *mut (), tp: TypePackId) {
  // Safety: pack 侧孪生分支——同一 owner 装配不变量、同一 TxnLog::empty() 单例与
  // 已装配 vtable 前置条件（基类 traverse 期间由宿主独占驱动，无并发访问该对象）。
  unsafe { (*(owner as *mut Widen)).base.found_dirty_type_pack_id(tp) }
}

fn widen_ignore_children_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 回转合法（唯一装配点，宿主在遍历期间存活独占）。widen_ignore_children
  // 只经安全的 get_type::get::<T> 门面读 ty 指向的 arena 节点判别变体，不写任何状态；
  // ty 由 Tarjan 遍历从 nodes/edges 快照取出，始终指向存活节点。
  unsafe { (*(owner as *mut Widen)).widen_ignore_children(ty) }
}

fn widen_ignore_children_tp(_owner: *mut (), _tp: TypePackId) -> bool {
  false
}

impl Widen {
  pub fn widen_widen(arena: Handle<TypeArena>, builtin_types: Handle<BuiltinTypes>) -> Self {
    Widen {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      builtin_types,
    }
  }

  pub(crate) fn install_substitution_vtable(&mut self) {
    let owner = self as *mut Widen as *mut ();
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(widen_is_dirty_ty),
      is_dirty_tp: Some(widen_is_dirty_tp),
      clean_ty: Some(widen_clean_ty),
      clean_tp: Some(widen_clean_tp),
      found_dirty_ty: Some(widen_found_dirty_ty),
      found_dirty_tp: Some(widen_found_dirty_tp),
      ignore_children_ty: Some(widen_ignore_children_ty),
      ignore_children_tp: Some(widen_ignore_children_tp),
      ignore_children_visit_ty: Some(widen_ignore_children_ty),
      ignore_children_visit_tp: Some(widen_ignore_children_tp),
    };
  }

  pub fn widen_type(&mut self, ty: TypeId) -> TypeId {
    self.install_substitution_vtable();
    self.base.substitute_type_id(ty).unwrap_or(ty)
  }

  pub fn widen_type_pack(&mut self, tp: TypePackId) -> TypePackId {
    self.install_substitution_vtable();
    self.base.substitute_type_pack_id(tp).unwrap_or(tp)
  }
}
