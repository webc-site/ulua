//! §2（裸指针 → Rust 类型）CFG `InstrId` 句柄注册表（任务 #17 续，与
//! [`crate::records::sym_def_registry`] 的 `SymDefId`、
//! [`crate::records::block_registry`] 的 `BlockId` 句柄同一先例形状）。
//!
//! C++ `using InstrId = NotNull<Instruction>`（`Analysis/include/Luau/ControlFlowGraph.h:38`）
//! 把 `CFGAllocator::instructions` arena 节点地址当身份令牌横传指令发射
//! （`emit`）、Join 补全（`incomplete_joins`）与转储全链。Rust 侧改为
//! [`InstrId`]（u32 句柄）+ 本模块的单点注册表：裸 `*mut Instruction` 只在
//! [`register_instruction`] 一处进入系统（节点由 `CfgAllocator::instructions`
//! 的 bump 块保活、地址不移动），业务逻辑只持有/比较/散列句柄，永不解引用
//! 指针；读回指令走 [`resolve_instruction`]（只读，供转储与 `get_if` 甄别），
//! 构建期写回（`fillJoinOperands` 追加操作数）经
//! [`resolve_instruction_mut`]，其 `unsafe` 依赖下列类型级契约。
//!
//! # Safety（注册表级契约，与迁移前的隐含前提逐字对应）
//! 1. 注册只发生在 `CfgAllocator::new_instruction` 分配点：节点存活于
//!    instructions bump 块中，块地址不移动、节点直到宿主 `CfgAllocator`
//!    释放前始终有效——句柄解引用与原先的 `NotNull<Instruction>` 解引用同
//!    一前提；
//! 2. 线程内注册表（`thread_local`）：CFG 构建（`emit`/`seal`/`fillJoinOperands`）
//!    与转储（`dump_instruction` 族）在同一分析会话线程上串行驱动（cpp 中
//!    InstrId 亦不跨线程解引用），故句柄的分配与解析必落在同一线程；异线程
//!    句柄查表越界返回 `None`，比 cpp 的悬垂解引用更保守；
//! 3. 句柄 id 单调增长、永不回收复用：allocator 释放后旧句柄至多解析到陈旧
//!    节点（cpp 悬垂指针同效），但绝不会与新 allocator 的节点混淆——身份
//!    隔离强于原先的地址复用语义；id 0 保留为「空哨兵」占位（与
//!    `SymDefId::NULL` 同构，对应迁移前 `DenseHashMap` 以 null 指针占位的
//!    形态），永不入库。
//!
//! 与 `SymDef`（分配后仅读）不同，`Instruction::Join` 的 operands 在 seal 期
//! 补全写回，故本模块额外提供 [`resolve_instruction_mut`]：同一时刻至多一个
//! 存活可变借用由「单分析线程、builder 独占 `&mut self`、写回点顺序借用不
//! 重叠」的原先裸指针纪律保证，与迁移前 `&mut *ptr` 的别名形态逐位同构。

use core::cell::RefCell;

use ulua_common::records::handle_registry::HandleRegistry;

use crate::type_aliases::instruction::Instruction;

/// `InstrId` 身份句柄（原 `InstrId = NotNull<Instruction> = *mut Instruction`
/// 的类型化替代）。
///
/// `u32` 索引指向本模块线程内注册表；派生 `Eq + Hash + Ord` 令其可直接作
/// `BTreeSet` 成员——原先以 `Join*` 裸指针为元素的 `incomplete_joins` 值集
/// 逐一改持句柄。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstrId(pub u32);

impl InstrId {
  /// 空哨兵：与 `SymDefId::NULL`/`BlockId::NULL` 同构的占位身份。
  pub const NULL: InstrId = InstrId(0);

  /// 是否为空哨兵（原 `ptr::is_null()` 判据）。
  pub fn is_null(self) -> bool {
    self.0 == InstrId::NULL.0
  }
}

thread_local! {
  /// `id -> 节点地址` 映射（泛型骨架见 [`HandleRegistry`]）；下标 `i` 存
  /// `InstrId(i + 1)`，`InstrId(0)` 不入库。
  static INSTR_REGISTRY: RefCell<HandleRegistry<Instruction>> =
    const { RefCell::new(HandleRegistry::new()) };
}

/// 注册点（唯一入口）：把 `CfgAllocator::new_instruction` 刚分配的节点地址
/// 收进注册表并发放句柄。
pub(crate) fn register_instruction(inst: *mut Instruction) -> InstrId {
  InstrId(INSTR_REGISTRY.with(|reg| reg.borrow_mut().register(inst as *const Instruction)))
}

/// 解析点（只读出口）：句柄 → 指令只读视图。空哨兵与越界句柄返回 `None`。
///
/// 返回借用的生命周期刻意不受约束（与 `resolve_sym_def`/`resolve_block`
/// 同一纪律）：与原裸指针解引用的借用检查行为逐位同构。变体甄别经
/// `InstructionMember::get_if` 在此只读视图上完成。
pub fn resolve_instruction<'a>(id: InstrId) -> Option<&'a Instruction> {
  if id.is_null() {
    return None;
  }
  INSTR_REGISTRY.with(|reg| {
    // SAFETY: 契约 1——非 None 时指针指向 instructions bump 块内存活、对齐、
    // 完整的 `Instruction`，块地址不移动；分析会话单线程驱动（契约 2），
    // 只读借用无并存别名。
    unsafe { reg.borrow().resolve(id.0) }
  })
}

/// 解析点（构建期写回出口）：句柄 → 指令可变视图。空哨兵与越界句柄返回
/// `None`。
///
/// # 可变借用纪律（对应迁移前的裸指针写回前提）
/// 同一句柄至多存在一个存活 `&mut`：builder 全程持 `&mut self` 单线程串行
/// 驱动，唯一写回点 `fill_join_operands` 在 `read_variable` 递归返回后才
/// 追加操作数、借用顺序不重叠——与迁移前 `&mut *j` 的别名形态逐位同构
/// （见模块契约 1、2）。
pub(crate) fn resolve_instruction_mut<'a>(id: InstrId) -> Option<&'a mut Instruction> {
  if id.is_null() {
    return None;
  }
  INSTR_REGISTRY.with(|reg| {
    // SAFETY: 契约 1——指针指向 instructions bump 块内存活、对齐、完整的
    // `Instruction`，块地址不移动；契约 2——单分析线程且调用点遵守上面的
    // 可变借用纪律，同一时刻至多一个并存 `&mut`，无数据竞争。
    unsafe { reg.borrow().resolve_mut(id.0) }
  })
}
