//! §2（裸指针 → Rust 类型）`BlockedConstraintId` 约束顶点句柄注册表（与
//! [`crate::records::sym_def_registry`] 的 `SymDefId`、
//! [`crate::records::block_registry`] 的 `BlockId`、
//! [`crate::records::instr_registry`] 的 `InstrId` 同一先例形状）。
//!
//! C++ `BlockedConstraintId = Variant<NotNull<Type>, NotNull<TypePack>,
//! NotNull<Constraint>>`（`Analysis/include/Luau/ConstraintGraph.h:16-19`）把
//! `Constraint` 的 Box 堆地址当身份令牌横传依赖表（`DenseHashMap` 键面）、
//! 解阻塞链（`unblockConstraint`）与转储全链；本 crate 早前以
//! `*const Constraint` 直存该分支。Rust 侧改为 [`ConstraintId`]（u32 句柄）+
//! 本模块注册表：裸 `*const Constraint` 只在 [`register_constraint`] 一处进入
//! 系统（分配点 `ConstraintSolver::push_constraint` 以 Box 堆地址登记；
//! find-or-insert 幂等，故从 solver `constraints`/`unsolved_constraints`/
//! `deprecated_dependencies` 等既有指针数据槽回取的地址映射回同一句柄，
//! 「同址 ⇔ 同句柄」双射与迁移前的指针键判等逐位同构），业务逻辑只
//! 持有/比较/散列句柄，永不解引用指针；读回约束节点的唯一路径是
//! [`resolve_constraint`]（只读，供 `to_string`/dump 与 `hasUnsolvedDependencies`
//! 的 `PrimitiveType` 甄别），其 `unsafe` 依赖下列类型级契约。
//!
//! # Safety（注册表级契约，与迁移前的隐含前提逐字对应）
//! 1. 节点存活：约束由 `ConstraintSolver::solver_constraints`（`Vec<Box<
//!    Constraint>>`）持有，Box 堆址稳定、push 移动 Box 不改节点地址，且求解
//!    会话内无人释放——句柄解引用与原先 `*const Constraint` 解引用同一前提；
//!    会话结束整体 drop 后旧句柄至多解析到陈旧节点（cpp 悬垂指针同效）；
//! 2. 线程内注册表（`thread_local`）：约束求解与转储（`dumpBlocked`/
//!    `dumpWith`/`toString` 族）在同一分析会话线程上串行驱动（cpp 中约束
//!    指针亦不跨线程解引用），句柄的登记与解析必落在同一线程；异线程
//!    句柄查表越界返回 `None`，比 cpp 的悬垂解引用更保守；
//! 3. 句柄 id 单调增长、永不回收复用；id 0 保留为「空哨兵」（与
//!    `SymDefId::NULL`/`BlockId::NULL` 同构，对应迁移前
//!    `BlockedConstraintId::V2(null())` 的缺省键面），永不入库。
//!
//! 与 `Block`/`Instruction` 不同，本注册表只读（`Constraint` 的可变访问经
//! solver 自身的 `&mut`/`NonNull` 数据槽进行，不经句柄面），故不提供
//! `resolve_*_mut`。

use core::cell::RefCell;

use ulua_common::records::handle_registry::HandleRegistry;

use crate::records::constraint::Constraint;

/// `ConstraintId` 身份句柄（原 `BlockedConstraintId` V2 分支
/// `*const Constraint` 的类型化替代）。
///
/// `u32` 索引指向本模块线程内注册表；派生 `Eq + Hash` 令其可直接作
/// `Variant3` 成员进入 `DenseHashMap`/`ConstraintList` 键面——判等即 id 判等，
/// 与迁移前「指针判等」经 find-or-insert 双射逐位等价。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstraintId(pub u32);

impl ConstraintId {
  /// 空哨兵：对应迁移前 `BlockedConstraintId::V2(null())` 的键面缺省。
  pub const NULL: ConstraintId = ConstraintId(0);

  /// 是否为空哨兵（原 `ptr::is_null()` 判据）。
  pub fn is_null(self) -> bool {
    self.0 == ConstraintId::NULL.0
  }
}

thread_local! {
  /// `指针地址 -> 首发句柄` 双射存储（泛型骨架见 [`HandleRegistry`]，
  /// 本表走 find-or-insert 幂等的 `intern` 路径）：`nodes[i]` 存
  /// `ConstraintId(i + 1)`，`ConstraintId(0)` 不入库（见模块契约 3）；
  /// 反查索引保证同址 ⇒ 同句柄。
  static BLOCKED_CONSTRAINT_REGISTRY: RefCell<HandleRegistry<Constraint>> =
    const { RefCell::new(HandleRegistry::new()) };
}

/// 登记点（唯一入口）：`ConstraintSolver::push_constraint` 在 Box 分配处调用；
/// 各 `BlockedConstraintId::V2` 构造点经同一函数把既有指针数据槽的地址折算回
/// 句柄（find-or-insert 幂等，重复登记返回原句柄）。空指针映射为空哨兵。
pub(crate) fn register_constraint(c: *const Constraint) -> ConstraintId {
  if c.is_null() {
    return ConstraintId::NULL;
  }
  ConstraintId(BLOCKED_CONSTRAINT_REGISTRY.with(|reg| reg.borrow_mut().intern(c)))
}

/// 解析点（只读出口）：句柄 → 约束节点共享视图。空哨兵与越界句柄返回
/// `None`。
///
/// 返回借用的生命周期刻意不受约束（与 `resolve_sym_def`/`resolve_block`
/// 同一纪律）：与原裸指针解引用的借用检查行为逐位同构。
pub(crate) fn resolve_constraint<'a>(id: ConstraintId) -> Option<&'a Constraint> {
  if id.is_null() {
    return None;
  }
  BLOCKED_CONSTRAINT_REGISTRY.with(|reg| {
    // SAFETY: 契约 1——非 None 时指针指向 solver 存活 `Box<Constraint>` 的堆址，
    // 对齐且完整、地址不移动；契约 2——分析会话单线程驱动，只读借用无并存
    // 别名（写回仍走 solver 自身的指针数据槽，与本句柄读窗口按 cpp 同款顺序
    // 串行，不重叠）。
    unsafe { reg.borrow().resolve(id.0) }
  })
}
