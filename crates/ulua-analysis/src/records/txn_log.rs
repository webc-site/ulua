//! §2（arena / 自引用图句柄化）：事务日志记录。
//!
//! 迁移前两处裸指针形态在此收敛：
//!
//! 1. **parent 链**：`parent: *mut TxnLog` → `Option<Handle<TxnLog>>`。子日志
//!    （子 Unifier）只沿链**只读**查 pending，故复用 [`Handle`] 这一已有最小壳，
//!    业务侧经 [`TxnLog::chain`] 迭代器拿到 `&TxnLog`，不再出现 `&*p` 解引用。
//! 2. **seen 栈自引用**：`owned_seen: Vec<..>` + `shared_seen: *mut Vec<..>` +
//!    `owned_seen_box: Option<Box<..>>` 三字段（结构体自引用裸地址，需 Box 钉址
//!    才成立）→ 单个 `Option<SeenStack>`，内部 `Rc<RefCell<Vec<..>>>`。
//!    root 日志持有、子日志共享同一栈，与 C++ `sharedSeen` 的接线一一同源。
//! 3. **对元身份**：seen 栈元素原为 `*const ()` 单元裸指针，现用 [`VisitKey`]
//!    地址 newtype（同「地址即身份」语义，无指针类型）。
//! 4. **pending 槽**：表值原为 `Box<PendingType>`，`pending(&self)` 经
//!    `from_ref(&T).cast_mut()` 派生可变裸指针供下游写穿（写穿共享引用）。现改
//!    [`PendingSlot`]（`Box<UnsafeCell<T>>`）：写权限由类型声明，`&self` 路线的
//!    解引用收口在 `pending_slot` 一处。

use alloc::{rc::Rc, vec::Vec};
use core::{
  cell::RefCell,
  fmt::{self, Debug, Formatter},
};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    arena_handle::Handle, pending_slot::PendingSlot, pending_type::PendingType,
    pending_type_pack::PendingTypePack, visit_key::VisitKey,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// seen 栈元素的节点身份（原 `*const ()` 单元裸指针）。
pub type TypeOrPackId = VisitKey;

/// 「已见对」栈：C++ `TxnLog::sharedSeen` 指向的那条 `std::vector`。
///
/// 借用（`shared`）与持有（`owned`）的区分决定 [`TxnLog`] 复制时的行为：
/// 持有者复制出**独立副本**（副本的 push/pop 不回写原栈），借用者复制只加
/// 引用计数——与迁移前「Box 深拷贝 / 裸指针别名」两条分支逐一等价。
#[derive(Clone)]
pub struct SeenStack {
  storage: Rc<RefCell<Vec<(TypeOrPackId, TypeOrPackId)>>>,
  /// true = 本日志持有该栈（原 `owned_seen_box: Some(..)`）。
  owned: bool,
}

impl SeenStack {
  /// 新建一条自有栈（`TxnLog::new` / mk_unifier 的顶层日志）。
  pub(crate) fn owned() -> Self {
    Self {
      storage: Rc::default(),
      owned: true,
    }
  }

  /// 借用同一条栈（子日志接线，原 `shared_seen = parent->sharedSeen`）。
  pub(crate) fn shared(&self) -> Self {
    Self {
      storage: Rc::clone(&self.storage),
      owned: false,
    }
  }

  /// [`TxnLog`] 复制时的语义：持有者深拷贝、借用者加引用。
  fn clone_for_log(&self) -> Self {
    if self.owned {
      Self {
        storage: Rc::new(RefCell::clone(&self.storage)),
        owned: true,
      }
    } else {
      self.shared()
    }
  }

  /// 是否已见过该对（C++ `std::find` 线性扫描，顺序与短路条件一致）。
  pub(crate) fn contains(&self, pair: &(TypeOrPackId, TypeOrPackId)) -> bool {
    self.storage.borrow().iter().any(|p| p == pair)
  }

  /// 栈顶（供 `pop_seen` 的 LIFO 断言使用）。
  pub(crate) fn last(&self) -> Option<(TypeOrPackId, TypeOrPackId)> {
    self.storage.borrow().last().copied()
  }

  /// 入栈。
  pub(crate) fn push(&self, pair: (TypeOrPackId, TypeOrPackId)) {
    self.storage.borrow_mut().push(pair);
  }

  /// 出栈（栈空时无操作，与 `Vec::pop` 一致）。
  pub(crate) fn pop(&self) {
    self.storage.borrow_mut().pop();
  }
}

impl Debug for SeenStack {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("SeenStack")
      .field("depth", &self.storage.borrow().len())
      .field("owned", &self.owned)
      .finish()
  }
}

#[derive(Debug)]
pub struct TxnLog {
  pub(crate) type_var_changes: DenseHashMap<TypeId, PendingSlot<PendingType>>,
  pub(crate) type_pack_changes: DenseHashMap<TypePackId, PendingSlot<PendingTypePack>>,
  /// 外层日志（子 Unifier 的 pending 链式查找用）；根日志为 `None`。
  /// 目标由宿主（父 Unifier）持有且比本子日志长寿，见 [`Handle`] 的类型级契约。
  parent: Option<Handle<TxnLog>>,
  /// 生效中的「已见对」栈：`None` 即本日志不做环检测（原 `shared_seen` null 哨兵）。
  seen: Option<SeenStack>,
  pub(crate) radioactive: bool,
}

impl TxnLog {
  /// 以指定 seen 栈与父链接构造一条空日志（各构造点的唯一收敛出口）。
  fn with_seen(seen: Option<SeenStack>, parent: Option<Handle<TxnLog>>) -> Self {
    Self {
      type_var_changes: DenseHashMap::default(),
      type_pack_changes: DenseHashMap::default(),
      seen,
      parent,
      radioactive: false,
    }
  }

  /// 顶层日志：自持一条 seen 栈，无父链接（对应 C++ `TxnLog log;`）。
  pub(crate) fn root() -> Self {
    Self::with_seen(Some(SeenStack::owned()), None)
  }

  /// 子日志：`pending` 沿父链上溯，seen 栈与父日志共享同一条（C++
  /// `TxnLog(TxnLog* parent)`）。父日志须比返回的子日志长寿（[`Handle`] 契约）。
  pub(crate) fn child_of(parent: &TxnLog) -> Self {
    let seen = parent.seen.as_ref().map(SeenStack::shared);
    Self::with_seen(seen, Some(Handle::from_ref(parent)))
  }

  /// 无环检测日志：seen 栈缺席（原 `shared_seen = nullptr` 形态），
  /// `have/push/pop_seen` 皆为 no-op。
  pub(crate) fn without_seen() -> Self {
    Self::with_seen(None, None)
  }

  /// 回滚快照用的日志：借用本日志的 seen 栈、不接父链
  /// （C++ `TxnLog inversed(sharedSeen)`）。
  pub(crate) fn inverse_of(source: &TxnLog) -> Self {
    Self::with_seen(source.seen.as_ref().map(SeenStack::shared), None)
  }

  /// 本日志的 seen 栈（`None` = 环检测关闭）。
  pub(crate) fn seen_stack(&self) -> Option<&SeenStack> {
    self.seen.as_ref()
  }

  /// 首次需要记录 seen 对时按需建栈（原 `push_seen` 里的懒分配分支）。
  pub(crate) fn seen_stack_mut(&mut self) -> &SeenStack {
    self.seen.get_or_insert_with(SeenStack::owned)
  }

  /// 自本日志起的父链迭代器（含自身）：`pending` 查找沿此链上溯。
  pub(crate) fn chain(&self) -> LogChain<'_> {
    LogChain { next: Some(self) }
  }
}

/// [`TxnLog::chain`] 的父链迭代器：自身 → 父 → 祖父 …，全程只读借用。
pub struct LogChain<'a> {
  next: Option<&'a TxnLog>,
}

impl<'a> Iterator for LogChain<'a> {
  type Item = &'a TxnLog;

  fn next(&mut self) -> Option<Self::Item> {
    let current = self.next?;
    self.next = current.parent.map(|handle| handle.get());
    Some(current)
  }
}

impl Clone for TxnLog {
  fn clone(&self) -> Self {
    // seen 栈按「持有者深拷贝、借用者共享」复制（迁移前 Box 深拷贝 /
    // 裸指针别名两条分支的等价形态），父链接按句柄值复制。
    Self {
      type_var_changes: self.type_var_changes.clone(),
      type_pack_changes: self.type_pack_changes.clone(),
      parent: self.parent,
      seen: self.seen.as_ref().map(SeenStack::clone_for_log),
      radioactive: self.radioactive,
    }
  }
}
