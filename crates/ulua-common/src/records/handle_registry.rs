//! §2（裸指针 → Rust 类型）通用句柄注册表 [`HandleRegistry`]。
//!
//! `ulua-analysis` 的六个句柄注册表（`DefId`/`SymDefId`/`BlockId`/`InstrId`/
//! `ScopeId`/`ConstraintId`）共享同一骨架：`thread_local! { RefCell<Vec<槽位>> }`，
//! 加上 u32 单调 id 发放以及 `id -> 节点视图` 解析，部分表附带 `地址 -> id` 反查
//! （find-or-insert 幂等）。本模块把该骨架泛型化为 [`HandleRegistry`]，各域
//! 只需一行实例化加各自的 id newtype 薄封装；注册表本体仍留在各域的
//! `thread_local` 包装内（会话/线程边界不变），为后续把注册表从 `thread_local`
//! 提升为会话实体的并发化铺路。
//!
//! # 槽位形态 [`Slot`]
//! * `Slot::Ptr(*const T)`：节点由外部宿主（arena bump 块、solver 的
//!   `Vec<Box<T>>` 等）保活，注册表只抄地址；
//! * `Slot::Shared(Arc<T>)`：注册表收取 Arc 强引用保活（`Scope` 形态），
//!   宿主析构后节点仍存活。
//!
//! # id 契约（与迁移前逐表隐含前提对应，语义不变）
//! 1. 判等即 id 判等：id 由 `T` 的堆地址（裸指针或 Arc 堆块地址）首发登记，
//!    find-or-insert 路径（[`HandleRegistry::intern`] / [`HandleRegistry::intern_arc`]）
//!    经反查表保证「同址 ⇔ 同句柄」；
//! 2. id 单调增长、永不回收复用；id 0 为「空哨兵」，永不入库（各域 newtype
//!    的 `NULL` 约定），[`HandleRegistry::resolve_ptr`] 对 0 直接返回 `None`；
//! 3. 线程内注册表：本类型刻意不做 `Send`/`Sync` 论证，`*const T` 槽位本就
//!    非 `Send`——由各域 `thread_local!` 包装保证登记与解析同线程，异线程句柄
//!    查表越界返回 `None`。
//!
//! 节点存活前提（注册表级契约 1 的「地址指向的 `T` 仍然存活」部分）由调用方
//! 各域文档保证，与原先逐表成立条件逐字相同；故解析三函数均为 `unsafe fn`，
//! SAFETY 论证保留在域侧调用点。

use alloc::{sync::Arc, vec::Vec};

use crate::collections::HashMap;

/// 注册表槽位：外部保活的裸地址，或注册表自持强引用的 Arc。
enum Slot<T> {
  Ptr(*const T),
  Shared(Arc<T>),
}

impl<T> Slot<T> {
  #[inline]
  fn as_ptr(&self) -> *const T {
    match self {
      Slot::Ptr(p) => *p,
      Slot::Shared(arc) => Arc::as_ptr(arc),
    }
  }
}

/// u32 单调句柄 → 节点地址的线程内注册表（契约见模块头）。
///
/// 下标 `i` 存句柄 id `i + 1`；id 0 为保留的空哨兵，永不入库。
pub struct HandleRegistry<T> {
  nodes: Vec<Slot<T>>,
  /// `地址 -> 首发句柄` 反查表：仅在首次 [`HandleRegistry::intern`] /
  /// [`HandleRegistry::intern_arc`] 时创建；纯追加域（arena 分配点注册）恒为
  /// `None`，不付哈希成本。
  ids: Option<HashMap<usize, u32>>,
}

impl<T> Default for HandleRegistry<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> HandleRegistry<T> {
  /// 空注册表（可求值，供 `thread_local!` 的 `const` 初始化器）。
  pub const fn new() -> Self {
    Self {
      nodes: Vec::new(),
      ids: None,
    }
  }

  /// 纯追加登记（唯一发放新句柄的路径）：把节点地址收进表尾并发放 id。
  ///
  /// 不去重——调用点须保证该地址此前未入库（arena 分配点即满足）。
  pub fn register(&mut self, ptr: *const T) -> u32 {
    self.nodes.push(Slot::Ptr(ptr));
    self.nodes.len() as u32
  }

  /// find-or-insert 登记（幂等）：地址已登记返回原句柄，否则追加并发放新句柄。
  ///
  /// 对应「从既有指针数据槽回取的地址映射回同一句柄」的接线点（约束顶点
  /// 等），保证同址 ⇔ 同句柄双射（契约 1）。
  pub fn intern(&mut self, ptr: *const T) -> u32 {
    let key = ptr as usize;
    if let Some(&id) = self.ids.as_ref().and_then(|ids| ids.get(&key)) {
      return id;
    }
    self.nodes.push(Slot::Ptr(ptr));
    let id = self.nodes.len() as u32;
    self.ids.get_or_insert_with(HashMap::new).insert(key, id);
    id
  }

  /// Arc 版 find-or-insert 登记（幂等）：按 `Arc::as_ptr` 地址查表，未登记则
  /// 收取 Arc 克隆入表（注册表自此保活节点）并发放新句柄。
  ///
  /// 对应 `Scope` 形态：注册表持强引用，宿主析构后句柄仍可解析。
  pub fn intern_arc(&mut self, arc: &Arc<T>) -> u32 {
    let ptr = Arc::as_ptr(arc);
    let key = ptr as usize;
    if let Some(&id) = self.ids.as_ref().and_then(|ids| ids.get(&key)) {
      return id;
    }
    self.nodes.push(Slot::Shared(arc.clone()));
    let id = self.nodes.len() as u32;
    self.ids.get_or_insert_with(HashMap::new).insert(key, id);
    id
  }

  /// 句柄 → 节点地址。空哨兵（0）与越界句柄返回 `None`。
  #[inline]
  pub fn resolve_ptr(&self, id: u32) -> Option<*const T> {
    if id == 0 {
      return None;
    }
    self.nodes.get((id - 1) as usize).map(Slot::as_ptr)
  }

  /// 句柄 → 节点只读视图（经 [`HandleRegistry::resolve_ptr`]）。空哨兵与越界
  /// 句柄返回 `None`。
  ///
  /// 返回借用的生命周期刻意不受约束（`'a` 由调用点决定）：与原裸指针解引用
  /// 的借用检查行为逐位同构。
  ///
  /// # Safety
  /// 契约 1——`resolve_ptr` 非 `None` 时地址指向存活、对齐、完整的 `T`（由各
  /// 域文档的存活前提保证）；契约 3——同线程驱动，只读借用无并存别名。
  pub unsafe fn resolve<'a>(&self, id: u32) -> Option<&'a T> {
    // SAFETY: 上条契约由本函数调用方（域侧 resolve 出口）逐一论证，SAFETY
    // 注释保留在域侧调用点。
    unsafe { self.resolve_ptr(id).map(|p| &*p) }
  }

  /// 句柄 → 节点可变视图。空哨兵与越界句柄返回 `None`。
  ///
  /// # Safety
  /// 在 [`HandleRegistry::resolve`] 的契约之外，另要求同一句柄至多存在一个
  /// 存活 `&mut`：调用点单线程串行驱动、顺序借用互不重叠（与迁移前
  /// `&mut *ptr` / `arc_as_mut` 的别名纪律逐位同构）。
  pub unsafe fn resolve_mut<'a>(&self, id: u32) -> Option<&'a mut T> {
    // SAFETY: 独占写前提由本函数调用方论证（同一句柄至多一个存活 `&mut`，
    // 单线程顺序借用不重叠）。
    unsafe { self.resolve_ptr(id).map(|p| &mut *(p as *mut T)) }
  }
}

#[cfg(test)]
mod tests {
  use alloc::{boxed::Box, sync::Arc};

  use super::HandleRegistry;

  struct Node(u32);

  #[test]
  fn register_resolve_roundtrip_and_sentinel() {
    let mut reg = HandleRegistry::new();
    let a = Box::into_raw(Box::new(Node(7)));
    // 纯追加：id 从 1 起（0 为空哨兵，永不入库），单调不复用。
    assert_eq!(reg.register(a), 1);
    assert_eq!(reg.resolve_ptr(0), None);
    // SAFETY: a 为本测试 Box 保活的存活、对齐节点，单线程只读。
    assert!(unsafe { reg.resolve(1) }.is_some());
    assert_eq!(reg.resolve_ptr(2), None);
    let _ = unsafe { Box::from_raw(a) };
  }

  #[test]
  fn intern_is_idempotent_per_address() {
    let mut reg = HandleRegistry::new();
    let a = Box::into_raw(Box::new(Node(1)));
    let b = Box::into_raw(Box::new(Node(2)));
    assert_eq!(reg.intern(a), 1);
    assert_eq!(reg.intern(a), 1);
    assert_eq!(reg.intern(b), 2);
    let _ = unsafe { Box::from_raw(a) };
    let _ = unsafe { Box::from_raw(b) };
  }

  #[test]
  fn intern_arc_keeps_node_alive_after_host_drop() {
    let mut reg: HandleRegistry<Node> = HandleRegistry::new();
    let arc = Arc::new(Node(9));
    let id = reg.intern_arc(&arc);
    assert_eq!(id, 1);
    // 幂等：同一 Arc 堆块地址恒得首发句柄。
    assert_eq!(reg.intern_arc(&arc), 1);
    drop(arc);
    // 注册表持强引用保活：宿主句柄 drop 后仍可解析。
    assert_eq!(unsafe { reg.resolve(id) }.map(|n| n.0), Some(9));
    // SAFETY: id 唯一、单线程、该 Node 仅由注册表持有且此前无并存 `&mut`。
    unsafe { reg.resolve_mut(id) }.unwrap().0 = 10;
    assert_eq!(unsafe { reg.resolve(id) }.map(|n| n.0), Some(10));
  }
}
