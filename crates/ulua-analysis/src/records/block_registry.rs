//! §2（裸指针 → Rust 类型）CFG `BlockId` 句柄注册表（任务 #17 续，与
//! [`sym_def_registry`] 的 `SymDefId` 句柄同一先例形状）。
//!
//! C++ `using BlockId = NotNull<Block>`（`Analysis/include/Luau/ControlFlowGraph.h`）
//! 把 `CFGAllocator::block` arena 节点地址当身份令牌横传块接线（preds/succs）、
//! builder 状态（sealed/incomplete-join 集合、currentBlock）与转储全链。Rust 侧
//! 改为 [`BlockId`]（u32 句柄）+ 本模块的单点注册表：裸 `*mut Block` 只在
//! [`register_block`] 一处进入系统（节点由 `CfgAllocator::block` 的 bump 块保活、
//! 地址不移动），业务逻辑只持有/比较/散列句柄，永不解引用指针；读回节点走
//! [`resolve_block`]（只读）／[`resolve_block_mut`]（构建期写回），其 `unsafe`
//! 依赖下列类型级契约。
//!
//! # Safety（注册表级契约，与迁移前的隐含前提逐字对应）
//! 1. 注册只发生在 `CfgAllocator::new_block` 分配点：节点存活于 block bump 块
//!    中，块地址不移动、节点直到宿主 `CfgAllocator` 释放前始终有效——句柄
//!    解引用与原先的 `NotNull<Block>` 解引用同一前提；
//! 2. 线程内注册表（`thread_local`）：CFG 构建（`makeCFG`/`lower`/`readVariable`）
//!    与转储（`dump_cfg` 族）在同一分析会话线程上串行驱动（cpp 中 BlockId 亦
//!    不跨线程解引用），故句柄的分配与解析必落在同一线程；异线程句柄查表
//!    越界返回 `None`，比 cpp 的悬垂解引用更保守；
//! 3. 句柄 id 单调增长、永不回收复用：allocator 释放后旧句柄至多解析到陈旧
//!    节点（cpp 悬垂指针同效），但绝不会与新 allocator 的节点混淆——身份
//!    隔离强于原先的地址复用语义；id 0 保留为「空哨兵」（对应原先
//!    `CFGBuilder::newBlock` 可省略的 `pred = nullptr` 缺省语义，该缺省在调用
//!    侧已改 [`Option<BlockId>`] 表达），永不入库。
//!
//! 与 `SymDef`（分配后仅读）不同，`Block` 的 preds/succs/instructions/
//! reaching-definitions 在构建期持续写回，故本模块额外提供
//! [`resolve_block_mut`]：同一时刻至多一个存活可变借用由「单分析线程、
//! builder 独占 `&mut self`、调用点顺序借用不重叠」的原先裸指针纪律保证，
//! 与迁移前 `&mut *ptr` 的别名形态逐位同构。

use core::cell::RefCell;

use ulua_common::records::{dense_hash_table::DenseDefault, handle_registry::HandleRegistry};

use crate::records::block::Block;

/// `BlockId` 身份句柄（原 `BlockId = NotNull<Block> = *mut Block` 的类型化替代）。
///
/// `u32` 索引指向本模块线程内注册表；派生 `Eq + Hash + Ord` 令其可直接作
/// `DenseHashMap`/`DenseHashSet`/`BTreeSet` 成员与键——原先以指针值为键/集合的
/// `sealed_blocks`、`incomplete_joins` 与各 preds/succs 向量逐一改持句柄。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(pub u32);

impl BlockId {
  /// 空哨兵：对应迁移前 `null_mut()` 的「无前驱/缺省 pred」表达。
  pub const NULL: BlockId = BlockId(0);

  /// 是否为空哨兵（原 `ptr::is_null()` 判据）。
  pub fn is_null(self) -> bool {
    self.0 == BlockId::NULL.0
  }
}

/// `DenseHashMap`/`DenseHashSet` 空槽占位：id 0 永不入库（见模块契约 3），
/// 与迁移前指针值的 `null_mut()` 占位同构。
impl DenseDefault for BlockId {
  fn dense_default() -> Self {
    BlockId::NULL
  }
}

thread_local! {
  /// `id -> 节点地址` 映射（泛型骨架见 [`HandleRegistry`]）；下标 `i` 存
  /// `BlockId(i + 1)`，`BlockId(0)` 不入库。
  static BLOCK_REGISTRY: RefCell<HandleRegistry<Block>> = const { RefCell::new(HandleRegistry::new()) };
}

/// 注册点（唯一入口）：把 `CfgAllocator::new_block` 刚分配的节点地址收进
/// 注册表并发放句柄。
pub(crate) fn register_block(block: *mut Block) -> BlockId {
  BlockId(BLOCK_REGISTRY.with(|reg| reg.borrow_mut().register(block as *const Block)))
}

/// 解析点（只读出口）：句柄 → 节点只读视图。空哨兵与越界句柄返回 `None`。
///
/// 返回借用的生命周期刻意不受约束（与 [`sym_def_registry::resolve_sym_def`]
/// 同一纪律）：与原裸指针解引用的借用检查行为逐位同构。
pub fn resolve_block<'a>(id: BlockId) -> Option<&'a Block> {
  if id.is_null() {
    return None;
  }
  BLOCK_REGISTRY.with(|reg| {
    // SAFETY: 契约 1——非 None 时指针指向 block bump 块内存活、对齐、完整的
    // `Block`，块地址不移动；分析会话单线程驱动（契约 2），只读借用无并存
    // 别名。
    unsafe { reg.borrow().resolve(id.0) }
  })
}

/// 解析点（构建期写回出口）：句柄 → 节点可变视图。空哨兵与越界句柄返回 `None`。
///
/// # 可变借用纪律（对应迁移前的裸指针写回前提）
/// 同一句柄至多存在一个存活 `&mut`：builder 全程持 `&mut self` 单线程串行
/// 驱动，写回点（preds/succs 接线、指令追加、reaching definitions）顺序借用、
/// 互不重叠——与原先 `&mut *ptr` 的别名形态逐位同构（见模块契约 1、2）。
pub(crate) fn resolve_block_mut<'a>(id: BlockId) -> Option<&'a mut Block> {
  if id.is_null() {
    return None;
  }
  BLOCK_REGISTRY.with(|reg| {
    // SAFETY: 契约 1——指针指向 block bump 块内存活、对齐、完整的 `Block`，
    // 块地址不移动；契约 2——单分析线程且调用点遵守上面的可变借用纪律，
    // 同一时刻至多一个并存 `&mut`，无数据竞争。
    unsafe { reg.borrow().resolve_mut(id.0) }
  })
}
