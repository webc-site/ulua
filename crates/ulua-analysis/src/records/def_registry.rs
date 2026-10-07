//! §2（裸指针 → Rust 类型）DefId 句柄注册表。
//!
//! C++ `using DefId = NotNull<const Def>`（`Analysis/include/Luau/Def.h:16`）把
//! arena 节点地址当身份令牌横传约 60 个文件。Rust 侧改为 [`DefId`]（u32 句柄）+
//! 本模块的单点注册表：裸 `*const Def` 只在 [`register_def`] 一处进入系统（节点
//! 由宿主 `DefArena` 的 bump 块保活、地址不移动），业务逻辑只持有/比较/散列
//! 句柄，永不解引用指针；唯一读回节点的路径是 [`resolve_def`]，其 `unsafe` 依赖
//! 下列类型级契约。
//!
//! # Safety（注册表级契约，与迁移前的隐含前提逐字对应）
//! 1. 注册只发生在 `DefArena` 分配点：节点存活于 arena bump 块中，块地址不
//!    移动、节点直到宿主 `Module` 释放前始终有效——句柄解引用与原先的
//!    `NotNull<const Def>` 解引用同一前提；
//! 2. 线程内注册表（`thread_local`）：DFG 构建与细化求解在同一分析会话线程上
//!    串行驱动（cpp 中 `DataFlowGraph`/`ConstraintGenerator` 亦不跨线程解引用
//!    DefId），故句柄的分配与解析必落在同一线程；异线程句柄查表越界返回
//!    `None`，比 cpp 的悬垂解引用更保守；
//! 3. 句柄 id 单调增长、永不回收复用：模块释放后旧句柄至多解析到陈旧节点
//!    （cpp 悬垂指针同效），但绝不会与新模块的节点混淆——身份隔离强于原先的
//!    地址复用语义；id 0 保留为「空哨兵」（对应原先可为 null 的 refinement
//!    键默认值），永不入库。

use core::cell::RefCell;

use ulua_common::records::{dense_hash_table::DenseDefault, handle_registry::HandleRegistry};

use crate::{records::def::Def, type_aliases::variant::VariantMember};

/// `DefId` 身份句柄（原 `*const Def`/`*const ()` 令牌的类型化替代）。
///
/// `u32` 索引指向本模块线程内注册表；派生 `Eq + Hash + Ord` 令其可直接作
/// `DenseHashMap`/`BTreeMap` 键——原先以指针值为键的映射逐一改以句柄为键。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefId(pub u32);

impl DefId {
  /// 空哨兵：对应迁移前 `null()` 的 refinement 键/`DataFlowResult` 默认值。
  pub const NULL: DefId = DefId(0);

  /// 是否为空哨兵（原 `ptr::is_null()` 判据）。
  pub fn is_null(self) -> bool {
    self.0 == DefId::NULL.0
  }
}

/// `DenseHashMap`/`DenseHashSet` 空槽占位：id 0 永不入库（见模块契约 3），
/// 与迁移前指针表的 `null()` 占位同构。
impl DenseDefault for DefId {
  fn dense_default() -> Self {
    DefId::NULL
  }
}

thread_local! {
  /// `id -> 节点地址` 映射（泛型骨架见
  /// [`HandleRegistry`]）；下标 `i` 存 `DefId(i + 1)`，`DefId(0)` 不入库。
  static DEF_REGISTRY: RefCell<HandleRegistry<Def>> = const { RefCell::new(HandleRegistry::new()) };
}

/// 注册点（唯一入口）：把 `DefArena` 刚分配的节点地址收进注册表并发放句柄。
pub(crate) fn register_def(def: *const Def) -> DefId {
  DefId(DEF_REGISTRY.with(|reg| reg.borrow_mut().register(def)))
}

/// 解析点（唯一出口）：句柄 → 节点地址。空哨兵与越界句柄返回 `None`。
///
/// 返回裸指针是刻意的：`get_def_id` 的变体下转与 `resolve_captures` 对
/// `Phi.operands` 的原地写入都需要地址语义；解引用安全契约见模块头。
fn resolve_def(id: DefId) -> Option<*const Def> {
  if id.is_null() {
    return None;
  }
  DEF_REGISTRY.with(|reg| reg.borrow().resolve_ptr(id.0))
}

/// 以 `&Def` 只读视图解析句柄（契约同 [`resolve_def`]，命中即存活节点）。
///
/// 返回借用的生命周期刻意不受约束（与本仓库 [`Handle`](crate::records::arena_handle::Handle)
/// 的 `get` 同一纪律）：与原裸指针解引用的借用检查行为逐位同构。
pub(crate) fn def_ref<'a>(id: DefId) -> Option<&'a Def> {
  // SAFETY: 契约 1——非 None 时指针指向 arena bump 块内存活、对齐、完整的
  // `Def`，块地址不移动；分析会话单线程驱动（契约 2），只读借用无并存别名。
  unsafe { resolve_def(id).and_then(|p| p.as_ref()) }
}

/// cpp `template<typename T> const T* get(DefId)` 的引用化：句柄 → 变体成员
/// 只读视图（`Cell`/`Phi`），未命中或空哨兵返回 `None`。业务逻辑经此读写
/// def 节点，不再接触任何裸指针。
pub fn def_as<'a, T: VariantMember>(id: DefId) -> Option<&'a T> {
  def_ref(id).and_then(|node| T::get_if(&node.v))
}

/// 变体成员的可写视图：对应 cpp 中 `const_cast` 后原地改写 arena 节点的
/// 唯一合法场景（`resolve_captures` 填充 `Phi.operands`）。
///
/// # Safety 说明
/// 与 [`resolve_def`] 同一契约，另要求调用点为该节点的独占写路径（构建期
/// 单线程、无并存借用）——与原裸指针写入处的隐含前提逐字一致。
pub fn def_as_mut<'a, T: VariantMember>(id: DefId) -> Option<&'a mut T> {
  // SAFETY: 契约 1 保证指针存活、对齐；独占写契约由本函数文档要求，调用点
  // （构建期 `resolve_captures`）满足。返回 `&'static mut` 与原 `&mut *ptr`
  // 同构，借用不受约束属本仓库 arena 句柄既有纪律（见 arena_handle.rs）。
  unsafe { resolve_def(id).map(|p| &mut *(p as *mut Def)) }
    .and_then(|node| T::get_if_mut(&mut node.v))
}
