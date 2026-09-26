//! §2（裸指针 → Rust 类型）CFG `SymDefId` 句柄注册表（任务 #17，与
//! [`def_registry`] 的 `DefId` 句柄同一先例形状）。
//!
//! C++ `using DefId = NotNull<Definition>`（`Analysis/include/Luau/ControlFlowGraph.h:31`，
//! `Definition = SymDef`）把 `CFGAllocator::defs` arena 节点地址当身份令牌横传
//! DFG 构建、refinement 命题与转储全链。Rust 侧改为 [`SymDefId`]（u32 句柄）+
//! 本模块的单点注册表：裸 `*const SymDef` 只在 [`register_sym_def`] 一处进入
//! 系统（节点由 `CfgAllocator.defs` 的 bump 块保活、地址不移动），业务逻辑只
//! 持有/比较/散列句柄，永不解引用指针；唯一读回节点的路径是
//! [`resolve_sym_def`]，其 `unsafe` 依赖下列类型级契约。
//!
//! # Safety（注册表级契约，与迁移前的隐含前提逐字对应）
//! 1. 注册只发生在 `CfgAllocator::new_definition` 分配点：节点存活于 defs
//!    bump 块中，块地址不移动、节点直到宿主 `CfgAllocator` 释放前始终有效——
//!    句柄解引用与原先的 `NotNull<Definition>` 解引用同一前提；
//! 2. 线程内注册表（`thread_local`）：CFG 构建（`makeCFG`/`lower`/`readVariable`）
//!    与转储（`dump_cfg` 族）在同一分析会话线程上串行驱动（cpp 中 DefId 亦
//!    不跨线程解引用），故句柄的分配与解析必落在同一线程；异线程句柄查表
//!    越界返回 `None`，比 cpp 的悬垂解引用更保守；
//! 3. 句柄 id 单调增长、永不回收复用：allocator 释放后旧句柄至多解析到陈旧
//!    节点（cpp 悬垂指针同效），但绝不会与新 allocator 的节点混淆——身份
//!    隔离强于原先的地址复用语义；id 0 保留为「空哨兵」（对应原先
//!    `getReachingDefinition` 可为 null 的缺省语义），永不入库。

use core::cell::RefCell;

use ulua_common::records::{dense_hash_table::DenseDefault, handle_registry::HandleRegistry};

use crate::records::sym_def::SymDef;

/// `SymDefId` 身份句柄（原 `DefId = *mut SymDef`/`*const SymDef` 令牌的类型化替代）。
///
/// `u32` 索引指向本模块线程内注册表；派生 `Eq + Hash + Ord` 令其可直接作
/// `DenseHashMap`/`BTreeMap` 键——原先以指针值为键/值的映射逐一改以句柄为值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymDefId(pub u32);

impl SymDefId {
  /// 空哨兵：对应迁移前 `null_mut()` 的 `getReachingDefinition` 缺省值。
  pub const NULL: SymDefId = SymDefId(0);

  /// 是否为空哨兵（原 `ptr::is_null()` 判据）。
  pub fn is_null(self) -> bool {
    self.0 == SymDefId::NULL.0
  }
}

/// `DenseHashMap`/`DenseHashSet` 空槽占位：id 0 永不入库（见模块契约 3），
/// 与迁移前指针值的 `null_mut()` 占位同构。
impl DenseDefault for SymDefId {
  fn dense_default() -> Self {
    SymDefId::NULL
  }
}

thread_local! {
  /// `id -> 节点地址` 映射（泛型骨架见 [`HandleRegistry`]）；下标 `i` 存
  /// `SymDefId(i + 1)`，`SymDefId(0)` 不入库。
  static SYM_DEF_REGISTRY: RefCell<HandleRegistry<SymDef>> =
    const { RefCell::new(HandleRegistry::new()) };
}

/// 注册点（唯一入口）：把 `CfgAllocator::new_definition` 刚分配的节点地址收进
/// 注册表并发放句柄。
pub(crate) fn register_sym_def(def: *const SymDef) -> SymDefId {
  SymDefId(SYM_DEF_REGISTRY.with(|reg| reg.borrow_mut().register(def)))
}

/// 解析点（唯一出口）：句柄 → 节点只读视图。空哨兵与越界句柄返回 `None`。
///
/// 返回借用的生命周期刻意不受约束（与 [`def_registry`] 的 `def_ref` 及本仓库
/// [`Handle`](crate::records::arena_handle::Handle) 的 `get` 同一纪律）：与原
/// 裸指针解引用的借用检查行为逐位同构。`SymDef` 分配后仅读（无 `&mut` 写回
/// 路径），故不提供可变视图。
pub fn resolve_sym_def<'a>(id: SymDefId) -> Option<&'a SymDef> {
  if id.is_null() {
    return None;
  }
  SYM_DEF_REGISTRY.with(|reg| {
    // SAFETY: 契约 1——非 None 时指针指向 defs bump 块内存活、对齐、完整的
    // `SymDef`，块地址不移动；分析会话单线程驱动（契约 2），只读借用无并存
    // 别名。
    unsafe { reg.borrow().resolve(id.0) }
  })
}
