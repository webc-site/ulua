//! §2（arena / 自引用图 → `Vec<节点> + 索引句柄`）：本 crate 私有的索引句柄层。
//!
//! 被照抄进 Rust 的 C++ arena 句柄有三类形态：`Type`/`TypePack` 节点裸指针
//! （`TypeId = *const Type`）、`PinnedStorage` 的 Box 堆址、以及 `*mut ()` 地址键。
//! 本模块给出第一类的 Rust 替代骨架：**节点存放在一个只追加的 `Vec<T>` 里，
//! 对外只发放 `u32` 槽号**。它相对 `PinnedStorage`（`Vec<Box<T>>` 钉住堆地址）
//! 的实质收益：
//!
//! - 句柄有效性不依赖「地址不移动」这条隐含前提，`Vec` 扩容搬迁节点不影响句柄；
//! - 少一层 Box 间接寻址与逐个分配，节点在内存上连续（缓存友好）；
//! - 双借用（同一数组既 `as_ptr` 读又 `&mut *` 写）在此形态下不可表达：
//!   取节点必须经 `&`/`&mut` 借用检查器，业务侧只能「先收集槽号、再按槽号读写」，
//!   这正是把 UB 隐患变成编译错误的地方。
//!
//! 与 `ulua-common` 的 `HandleRegistry`（地址 → u32 反查表，供「外部宿主保活的
//! 裸地址」用）互补：本模块**拥有**节点，故无需反查表，也不含任何 `unsafe`。
//! 后续 `TypeId` 索引化（W5/W6）时，各域用 `SlotArena<Type>` 实例化并保留
//! 自己的 newtype（如 `TypeId(u32)`），本层可直接复用。
//!
//! 建议后续把本模块上提到 `ulua-common`，与 `handle_registry` 并列为两种
//! 句柄骨架（自持 arena / 外部保活注册表）。

use alloc::vec::Vec;

use ulua_common::records::dense_hash_table::DenseDefault;

/// 保留哨兵槽号：`DenseHashMap` 空槽占位值（占用判定由位图负责，不参与比较），
/// 故必须取一个 [`SlotArena::push`] 永不发放的编号。
const NULL_SLOT: u32 = u32::MAX;

/// 槽位句柄上界：`u32` 槽号，单 arena 最多 `u32::MAX - 1` 个节点（分析会话内
/// 约束列表/节点量级远低于此；越界即分配异常，显式失败而非静默截断）。
const MAX_SLOTS: u32 = NULL_SLOT - 1;

/// 索引句柄：`SlotArena` 内节点的槽号，判等即槽号判等。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SlotId(u32);

impl SlotId {
  /// 空槽哨兵：仅作 `DenseHashMap` 值面的占位，永远不指向真实节点。
  pub const NULL: Self = Self(NULL_SLOT);

  /// 槽号 → 下标。
  #[inline]
  pub const fn index(self) -> usize {
    self.0 as usize
  }
}

// 句柄作为 `DenseHashMap` 值面时的空槽占位（原 `*mut ConstraintList` 的 null 占位
// 在此退化成一个不发放的槽号，语义等价：位图判定下该占位永不被读）。
impl DenseDefault for SlotId {
  fn dense_default() -> Self {
    Self::NULL
  }
}

/// 只追加的索引式节点存储（契约见模块头）。
#[derive(Debug, Clone)]
pub struct SlotArena<T> {
  nodes: Vec<T>,
}

// 手写而非 derive：derive 会强加 `T: Default` 约束，而图里存放的节点类型
// （如 `ConstraintList`）本就不需要实现 `Default`。
impl<T> Default for SlotArena<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> SlotArena<T> {
  pub fn new() -> Self {
    Self { nodes: Vec::new() }
  }

  /// 追加节点并发放槽号；槽号一经发放永不变、不复用。
  ///
  /// 槽位耗尽（`u32` 上限）时 panic——这是分配规模异常，不是可恢复错误。
  pub fn push(&mut self, value: T) -> SlotId {
    // DELIBERATE DEVIATION: cpp 侧依赖列表存储在 bump arena 上无容量上限，
    // 此处句柄为 u32 故显式设限并 panic（越界即分配异常，不可静默回绕），
    // 分析会话内约束列表量级远低于上限，正常路径行为一致。
    assert!(
      self.nodes.len() < MAX_SLOTS as usize,
      "SlotArena 槽位耗尽（u32 上限）"
    );
    self.nodes.push(value);
    SlotId(self.nodes.len() as u32 - 1)
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.nodes.len()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.nodes.is_empty()
  }

  /// 按槽号取节点。陈旧槽号（本 arena 未发放过）返回 `None`，不 panic。
  #[inline]
  pub fn get(&self, id: SlotId) -> Option<&T> {
    self.nodes.get(id.index())
  }

  /// 按槽号取节点的可变借用。
  #[inline]
  pub fn get_mut(&mut self, id: SlotId) -> Option<&mut T> {
    self.nodes.get_mut(id.index())
  }
}
