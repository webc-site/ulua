//! `txn_log` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。
//!
//! §2 句柄化后，本文件不再出现 `&*p` 解引用与自引用裸地址：父链走
//! [`TxnLog::chain`] 迭代器，seen 栈走 [`SeenStack`]（`Rc<RefCell<Vec>>`），
//! 可空句柄读写走 `arena_handle` 的唯一收口点。残余 `unsafe` 全部来自
//! `TypeId = *const Type` 这一 B 类 arena 裸句柄（`queue_*` 读节点），
//! 归 W5/W6 索引化时一并清零。

use alloc::boxed::Box;
use core::ptr::{from_ref, null_mut};
use std::sync::OnceLock;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::follow_option::FollowOption,
  functions::{
    follow_type::{FollowMapper, follow_full},
    follow_type_pack::{FollowPackMapper, follow_pack_full},
    get_mutable_txn_log::get_mutable_pending_type,
  },
  methods::txn_log_get_mutable::TxnLogGetMutable,
  records::{
    arena_handle::{alias, alias_opt, alias_ref},
    pending_type::PendingType,
    pending_type_pack::PendingTypePack,
    table_indexer::TableIndexer,
    table_type::TableType,
    txn_log::{TxnLog, TypeOrPackId},
    r#type::Type,
    type_pack_var::TypePackVar,
    visit_key::VisitKey,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// seen 对的两极规范化（C++ `haveSeen`/`pushSeen`/`popSeen` 共用的比较键）：
/// 较大者在前，与迁移前 `std::pair<const void*, const void*>` 的比较结果逐位等价。
fn sorted_pair(lhs: TypeOrPackId, rhs: TypeOrPackId) -> (TypeOrPackId, TypeOrPackId) {
  if lhs > rhs { (lhs, rhs) } else { (rhs, lhs) }
}

/// pending 表项 → 数据槽指针（纯指针算术，无解引用）：与原
/// `rep.as_ref() as *const T as *mut T` 逐位一致，目标由所属日志的 `Box` 保活。
fn slot_ptr<T>(rep: &T) -> *mut T {
  from_ref(rep).cast_mut()
}

/// 跳过 dead 墓碑条目，把 `Box<PendingType>` 表项降级为数据槽指针。
fn live_entry(entry: Option<&PendingType>) -> Option<*mut PendingType> {
  entry.filter(|rep| !rep.dead).map(slot_ptr)
}

impl TxnLog {
  pub(crate) fn bind_table(
    &mut self,
    ty: TypeId,
    new_bound_to: Option<TypeId>,
  ) -> *mut PendingType {
    // SAFETY: ty 是本日志所属会话 arena 中的存活 TypeId（C++ 断言同款前提）。
    let new_ty = unsafe { self.queue_type_id(ty) };

    // SAFETY: get_mutable_pending_type 是 C++ getMutable<TableType> 的对应物；
    // 未命中（原 null 哨兵）由 None 分支按 no-op 处理，与 cpp 判空短路一致。
    if let Some(table_type) = unsafe { get_mutable_pending_type::<TableType>(new_ty) } {
      table_type.bound_to = new_bound_to;
    }

    new_ty
  }
}

impl TxnLog {
  pub(crate) fn change_indexer(
    &mut self,
    ty: TypeId,
    indexer: Option<TableIndexer>,
  ) -> *mut PendingType {
    // SAFETY: 同 bind_table。
    let new_ty = unsafe { self.queue_type_id(ty) };

    // SAFETY: 同 bind_table。
    if let Some(table_type) = unsafe { get_mutable_pending_type::<TableType>(new_ty) } {
      table_type.indexer = indexer;
    }

    new_ty
  }
}

impl TxnLog {
  pub fn clear(&mut self) {
    self.type_var_changes.clear();
    self.type_pack_changes.clear();
  }
}

impl TxnLog {
  pub fn concat(&mut self, rhs: TxnLog) {
    // cpp `typeVarChanges[ty] = std::move(rep)`：operator[] 是无条件覆盖；
    // Box 无法从 &mut 迭代器里移出，这里退化成一次 clone。
    for (ty, rep) in rhs.type_var_changes.iter().filter(|(_, rep)| !rep.dead) {
      self.type_var_changes.insert(*ty, rep.clone());
    }

    for (tp, rep) in rhs.type_pack_changes.iter() {
      self.type_pack_changes.insert(*tp, rep.clone());
    }

    self.radioactive |= rhs.radioactive;
  }
}

/// 进程级空日志的 `Sync` 壳。
struct SyncTxnLog(Box<TxnLog>);
// SAFETY: 单例经 `OnceLock::get_or_init` 只初始化一次，此后仅外泄只读借用
// （`empty()` 返回 `*const TxnLog`），无并存可变别名，亦永不释放。
// `OnceLock<T>: Sync` 要求 `T: Send + Sync`，而 `TxnLog` 的 pending 表项含
// arena 裸句柄，派生不出这两条，故在此显式收口。
unsafe impl Sync for SyncTxnLog {}
unsafe impl Send for SyncTxnLog {}

impl TxnLog {
  /// 进程级只读空日志（C++ `TxnLog::empty()`）：无 pending 变更、seen 栈恒空。
  pub fn empty() -> *const TxnLog {
    static EMPTY_LOG: OnceLock<SyncTxnLog> = OnceLock::new();

    let wrapper = EMPTY_LOG.get_or_init(|| SyncTxnLog(Box::new(TxnLog::root())));
    Box::as_ref(&wrapper.0) as *const TxnLog
  }
}

impl TxnLog {
  /// C++ `TxnLog::follow(TypeId)`：follow 链上叠加本 log 的 pending 重定向。
  /// 映射器经 [`FollowMapper::Log`] 传入，context 裸指针往返的 unsafe 已消除。
  pub fn follow_type_id(&self, ty: TypeId) -> TypeId {
    follow_full(ty, FollowOption::Normal, FollowMapper::Log(self))
  }

  /// C++ `TxnLog::follow(TypePackId)`：follow 链上叠加本 log 的 pending 重定向。
  pub fn follow_type_pack_id(&self, tp: TypePackId) -> TypePackId {
    follow_pack_full(tp, FollowPackMapper::Log(self))
  }
}

impl TxnLog {
  /// C++ `TxnLog::get<T>(TID)` 的 Rust 惯用形态：命中变体返回引用，否则 `None`。
  /// 判空与解引用合并进 [`alias_opt`] 这一唯一收口点（null 哨兵 → `None`）；
  /// 可变性场景仍走 [`TxnLog::txn_log_get_mutable`]。
  pub fn txn_log_get<T, TID>(&self, ty: TID) -> Option<&T>
  where
    T: TxnLogGetMutable<TID> + 'static,
  {
    alias_opt(self.txn_log_get_mutable::<T, TID>(ty))
  }
}

impl TxnLog {
  #[inline]
  pub fn have_seen_type_id_type_id(&self, lhs: TypeId, rhs: TypeId) -> bool {
    self.have_seen_type_or_pack_id_type_or_pack_id(VisitKey::from_ptr(lhs), VisitKey::from_ptr(rhs))
  }

  #[inline]
  pub fn have_seen_type_pack_id_type_pack_id(&self, lhs: TypePackId, rhs: TypePackId) -> bool {
    self.have_seen_type_or_pack_id_type_or_pack_id(VisitKey::from_ptr(lhs), VisitKey::from_ptr(rhs))
  }

  pub fn have_seen_type_or_pack_id_type_or_pack_id(
    &self,
    lhs: TypeOrPackId,
    rhs: TypeOrPackId,
  ) -> bool {
    // seen 栈缺席即原 `shared_seen` null 哨兵：环检测关闭，恒未见。
    self
      .seen_stack()
      .is_some_and(|seen| seen.contains(&sorted_pair(lhs, rhs)))
  }
}

impl TxnLog {
  pub fn inverse(&self) -> TxnLog {
    // C++ `TxnLog inversed(sharedSeen)` —— 借用本日志的 seen 栈、不接父链。
    let mut inversed = TxnLog::inverse_of(self);

    for (ty, _rep) in self.type_var_changes.iter().filter(|(_, rep)| !rep.dead) {
      // SAFETY: alias_ref 的存活契约——键是本次事务登记时写入的 arena 节点
      // 句柄，TypedAllocator bump 块不移动节点、节点在会话 arena 存活期内常驻，
      // 而本 TxnLog 严格嵌于该 arena 存活期；只读取值快照（C++
      // `Type{entry.first->ty}` 同语义），单线程无并存写。
      let pending_ty = alias_ref(*ty).clone();
      inversed.type_var_changes.try_insert(
        *ty,
        Box::new(PendingType {
          pending: pending_ty,
          dead: false,
        }),
      );
    }

    for (tp, _rep) in self.type_pack_changes.iter() {
      // SAFETY: 同上——pack 键同为 bump arena 内存活节点，只读克隆作回滚快照。
      let pending_tp = alias_ref(*tp).clone();
      inversed.type_pack_changes.try_insert(
        *tp,
        Box::new(PendingTypePack {
          pending: pending_tp,
        }),
      );
    }

    inversed.radioactive = self.radioactive;

    inversed
  }
}

impl TxnLog {
  pub fn txn_log_is<T, TID>(&self, ty: TID) -> bool
  where
    T: TxnLogGetMutable<TID> + 'static,
  {
    self.txn_log_get::<T, TID>(ty).is_some()
  }
}

impl TxnLog {
  pub fn is_radioactive(&self) -> bool {
    self.radioactive
  }
}

impl TxnLog {
  /// 顶层日志：自持 seen 栈，析构即释放（cpp 成员 vector 的同位语义）。
  pub fn new() -> Self {
    TxnLog::root()
  }
}

impl Default for TxnLog {
  fn default() -> Self {
    Self::new()
  }
}

impl TxnLog {
  /// C++ `TxnLog::pending(TypeId)`：沿父链自本日志起取首个存活 pending 条目。
  /// 返回指针指向该日志 `Box<PendingType>` 的堆对象，日志存活期内地址稳定。
  pub fn pending_type_id(&self, ty: TypeId) -> *mut PendingType {
    // C++ 的 `LUAU_ASSERT(this != nullptr)` 在 Rust 由 `&self` 表达，无对应物。
    self
      .chain()
      .find_map(|log| live_entry(log.type_var_changes.find(&ty).map(|rep| &**rep)))
      .unwrap_or(null_mut())
  }

  /// C++ `TxnLog::pending(TypePackId)`：同 [`TxnLog::pending_type_id`] 的类型包形态
  /// （pack 条目无 dead 位，命中即有效）。
  pub fn pending_type_pack_id(&self, tp: TypePackId) -> *mut PendingTypePack {
    self
      .chain()
      .find_map(|log| log.type_pack_changes.find(&tp).map(|rep| &**rep))
      .map_or(null_mut(), slot_ptr)
  }
}

impl TxnLog {
  pub fn pop_seen_type_id_type_id(&mut self, lhs: TypeId, rhs: TypeId) {
    self.pop_seen_type_or_pack_id_type_or_pack_id(VisitKey::from_ptr(lhs), VisitKey::from_ptr(rhs));
  }

  pub fn pop_seen_type_pack_id_type_pack_id(&mut self, lhs: TypePackId, rhs: TypePackId) {
    self.pop_seen_type_or_pack_id_type_or_pack_id(VisitKey::from_ptr(lhs), VisitKey::from_ptr(rhs));
  }

  pub fn pop_seen_type_or_pack_id_type_or_pack_id(&mut self, lhs: TypeOrPackId, rhs: TypeOrPackId) {
    let pair = sorted_pair(lhs, rhs);

    if let Some(seen) = self.seen_stack() {
      LUAU_ASSERT!(seen.last() == Some(pair));
      seen.pop();
    }
  }
}

impl TxnLog {
  pub fn push_seen_type_id_type_id(&mut self, lhs: TypeId, rhs: TypeId) {
    self
      .push_seen_type_or_pack_id_type_or_pack_id(VisitKey::from_ptr(lhs), VisitKey::from_ptr(rhs));
  }

  pub fn push_seen_type_pack_id_type_pack_id(&mut self, lhs: TypePackId, rhs: TypePackId) {
    self
      .push_seen_type_or_pack_id_type_or_pack_id(VisitKey::from_ptr(lhs), VisitKey::from_ptr(rhs));
  }

  pub fn push_seen_type_or_pack_id_type_or_pack_id(
    &mut self,
    lhs: TypeOrPackId,
    rhs: TypeOrPackId,
  ) {
    // 缺席时按需新建自有栈（原 `Box::into_raw` 泄漏路径改为 Rc，随日志 drop）。
    self.seen_stack_mut().push(sorted_pair(lhs, rhs));
  }
}

impl TxnLog {
  pub fn replace_type_id_t<T>(&mut self, ty: TypeId, replacement: T) -> *mut PendingType
  where
    T: Into<Type>,
  {
    self.replace_type_id_type_item(ty, replacement.into())
  }

  pub(crate) fn replace_type_id_type_item(
    &mut self,
    ty: TypeId,
    replacement: Type,
  ) -> *mut PendingType {
    // SAFETY: queue_type_id 的「ty 指向存活 Type 节点」前置由调用方契约满足
    // （C++ ReplaceType 以 arena/pending 存活句柄入参）。
    let new_ty = unsafe { self.queue_type_id(ty) };
    // alias 的独占借用契约：表项由本 `&mut self` 日志独占，此刻该 pending 槽位
    // 仅此一处可变访问（&mut self 已排他了路径）。
    alias(new_ty).pending.reassign(&replacement);
    new_ty
  }

  pub(crate) fn replace_type_pack_id_type_pack_var(
    &mut self,
    tp: TypePackId,
    replacement: TypePackVar,
  ) -> *mut PendingTypePack {
    // SAFETY: 与 TypeId 侧对称——queue_type_pack_id 要求 tp 存活（C++
    // ReplaceTypePack 契约）。
    let new_tp = unsafe { self.queue_type_pack_id(tp) };
    // alias 契约同上：pack 表项此刻仅此一处可变访问。
    alias(new_tp).pending.reassign(&replacement);
    new_tp
  }
}
