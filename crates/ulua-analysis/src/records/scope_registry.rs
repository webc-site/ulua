//! §2（裸指针 → Rust 类型）`ScopeId` 句柄注册表（任务 #18，与 [`def_registry`] 的
//! `DefId`、[`sym_def_registry`] 的 `SymDefId` 同一先例形状）。
//!
//! C++ `children`/`parent` 以 `NotNull<Scope*>`/`Scope*` 裸指针横传（
//! `Analysis/include/Luau/Scope.h`），Rust 移植此前用 `Vec<*mut Scope>` 存子指针、
//! `Option<Arc<Scope>>` 存父指针，与 `unsafe impl Send/Sync for Scope` 手写实现
//! 互为表里（该 impl 现因 `TypeId`/`TypePackId` 仍为类型 arena 裸指针而暂留，
//! 见文末注）。本模块把「存储态」的 scope 树导航（`Scope::parent`/`Scope::children`）改为
//! [`ScopeId`]（u32 句柄）+ 线程内单点注册表：裸地址只在 [`register_scope`] 一处
//! （各 `Arc::new(Scope)` 创建点）进入系统，业务逻辑只持有/传递/比较句柄，
//! 解引用出口只有只读的 [`resolve_scope`] 与写回出口的 [`resolve_scope_mut`]
//! （后者对应迁移前经 `arc_as_mut` 的父/子 scope 原地写回，纪律与
//! [`block_registry`] 的 `resolve_block_mut` 同一先例）；持有 `ScopePtr` 的调用点
//! 反查句柄只经 [`intern_scope`]（地址即注册时入表的 Arc 堆块地址）。
//!
//! # Scope 的分配稳定性（调查结论，本重构的前提）
//! `Scope` 值从不存放在可增长的 `Vec<Scope>` 中：全部存活 scope 均为
//! `Arc<Scope>`（`type_aliases::scope_ptr_type::ScopePtr`）堆分配，所有权容器
//! （`Module.scopes: Vec<(Location, ScopePtr)>`、`ConstraintGenerator.scopes` 等）
//! 只搬运 `Arc` 克隆——`Vec` 扩容移动的是 Arc 控制块句柄槽位，绝不动 `Scope`
//! 本体地址，故句柄注册后指针恒定有效。原 `children: Vec<*mut Scope>` 正是借
//! 这条稳定性存活（元素来自 `Arc::as_ptr` 系写入句柄，由同树的 Arc 保活）；
//! 换成句柄后同一前提保留，但把全部 unsafe 解引用收进本模块，`Scope` 记录
//! 本体不再含 scope 树导航裸指针（`TypeId`/`TypePackId` 等类型 arena 裸指针
//! 字段的句柄化属后续任务）。
//!
//! # 存活前提：注册表收取 Arc 克隆（cpp 强引用 parent 的等价物）
//! cpp `Scope::parent` 是 `ScopePtr`（强引用），任一子 scope 恒保活其整条
//! 祖先链；仅被 parent 链可达、不在任何 scopes 容器里的 scope（如
//! `visitModuleRoot` 建后移交 `TypeFunctionRuntime::rootScope` 的模块级 type
//! function scope，运行时随检查会话局部量析构）在 cpp 中亦不释放。parent
//! 改存 `ScopeId` 后这一强引用从 `Scope` 记录中消失，故 [`register_scope`]
//! 收取 `Arc` 克隆入 [`SCOPE_REGISTRY`] 保活：注册表持有全部曾创建 scope
//! 的强引用，任何仍被句柄引用的节点必然存活，与 cpp 可达性逐位对应（表内
//! 至多多持有无句柄引用的死 scope——cpp 中它们也只在祖先链可达时存活，
//! 对外观察行为一致）。
//!
//! # Safety（注册表级契约，与迁移前的隐含前提逐字对应）
//! 1. 注册发生在 scope 的 `Arc::new` 创建点（[`register_scope`]）与仅持有父
//!    `ScopePtr` 的反查点（[`intern_scope`]，未注册即幂等补登记），注册表自此
//!    持有其 Arc 强引用直至线程结束（见上）：`Scope` 本体地址随 Arc 堆块
//!    稳定，[`resolve_scope`]/[`resolve_scope_mut`] 解引用恒有效，句柄一经
//!    发放绝不悬垂；[`intern_scope`] 反查以同一 Arc 堆块地址为键，命中即恒得
//!    该 scope 的同一句柄（地址稳定 ⇒ 键稳定），绝不静默发放野句柄；
//! 2. 线程内注册表（`thread_local`）：scope 树的构建（`childScope` 族）与
//!    遍历（`findInnermostScope`/`findNarrowestScopeContaining`/快照）在同一
//!    分析会话线程上串行驱动（cpp 中 Scope* 亦不跨线程解引用），异线程句柄
//!    查表越界返回 `None`，不会解引用他线程的 Arc 内存；
//! 3. 句柄 id 单调增长、永不回收复用：id 0 保留为空哨兵（`ScopeId::NULL`，
//!    永不入库，对应迁移前可为 null 的 `parent`/参数缺省语义），地址一经
//!    注册永不再被复用（注册表持 Arc 保活），旧会话的陈旧句柄至多解析到
//!    上一会话的真实 scope，绝不会与新 scope 身份混淆。

use core::cell::RefCell;

use ulua_common::records::handle_registry::HandleRegistry;

use crate::{records::scope::Scope, type_aliases::scope_ptr_type::ScopePtr};

/// `ScopeId` 身份句柄（原 `*mut Scope`/`*const Scope` 存储态令牌的类型化替代）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub u32);

impl ScopeId {
  /// 空哨兵：对应迁移前 null 的 scope 指针槽位，永不入库。
  pub const NULL: ScopeId = ScopeId(0);

  /// 是否为空哨兵（原 `ptr::is_null()` 判据）。
  pub fn is_null(self) -> bool {
    self.0 == ScopeId::NULL.0
  }
}

thread_local! {
  /// `id -> 注册时收取的 Arc<Scope>` 强引用表 + `Arc 内 Scope 地址 -> id` 反查
  /// 索引（泛型骨架见 [`HandleRegistry`] 的 Arc 槽位形态，两表由 `intern_arc`
  /// 同步写入；保活语义见「存活前提」节）。下标 `i` 存 `ScopeId(i + 1)`，
  /// `ScopeId(0)` 不入库。反查索引供仅持有 `ScopePtr` 的接线点（父链接、
  /// children 追加）反查句柄；键即表内 Arc 堆块地址，注册表保活 ⇒ 地址永不
  /// 回收复用（契约 3）。
  static SCOPE_REGISTRY: RefCell<HandleRegistry<Scope>> = const { RefCell::new(HandleRegistry::new()) };
}

/// 注册点（唯一入口）：把刚 `Arc::new` 出的 scope 的 Arc 克隆收进注册表
/// （强引用保活）并发放句柄。
///
/// 注册表自此与宿主（`Module`/`ConstraintGenerator`/`TypeFunctionRuntime` 等）
/// 共同持有该 `Scope`（契约 1）：宿主析构后 scope 仍存活，任何持有其句柄的
/// parent/children 边永不悬垂——逐字对应 cpp 强引用 `ScopePtr parent` 的
/// 祖先链保活语义。
///
/// 底层经 [`HandleRegistry::intern_arc`]（find-or-insert）：调用点均为 fresh
/// `Arc::new` 创建点（地址未入库 ⇒ 恒发放新句柄，与原纯追加形态逐位同构），
/// 若同一 Arc 被重复登记则幂等返回首发句柄（更强的双射保证，见契约 1）。
pub(crate) fn register_scope(arc: &ScopePtr) -> ScopeId {
  ScopeId(SCOPE_REGISTRY.with(|reg| reg.borrow_mut().intern_arc(arc)))
}

/// 反查点（Arc → 句柄，幂等）：把 `ScopePtr` 的堆地址映射回其句柄；地址尚未
/// 注册时当场执行 [`register_scope`] 补登记（收 Arc 克隆保活、发放新句柄）。
///
/// 仅做地址查表/入表，不解引用 `Scope` 内容；Arc 堆块地址稳定（契约 1）保证
/// 同一 scope 无论克隆多少次恒得首次发放的同一 id。供 `Scope::new` 等只持有
/// 父 `ScopePtr` 的接线点反查句柄——cpp 中把 Arc 交给子 scope 的 `ScopePtr
/// parent` 字段即自动保活该父 scope，本函数是这一语义在句柄态下的等价物，
/// 故绝不 panic：未登记者补登记而非拒绝。
pub(crate) fn intern_scope(arc: &ScopePtr) -> ScopeId {
  ScopeId(SCOPE_REGISTRY.with(|reg| reg.borrow_mut().intern_arc(arc)))
}

/// 解析点（只读出口）：句柄 → scope 只读视图。空哨兵与越界句柄返回 `None`。
///
/// 返回借用的生命周期刻意不受约束（与 [`def_registry`] 的 `def_ref` 及
/// [`sym_def_registry`] 的 `resolve_sym_def` 同一纪律）：与原裸指针解引用的
/// 借用检查行为逐位同构。children 的常规遍历全走本出口；仅当调用方需按
/// 迁移前 `arc_as_mut` 的原地写回形态改父/子 scope 字段时经
/// [`resolve_scope_mut`] 取可变视图。
pub fn resolve_scope<'a>(id: ScopeId) -> Option<&'a Scope> {
  if id.is_null() {
    return None;
  }
  SCOPE_REGISTRY.with(|reg| {
    // SAFETY: 契约 1——表内槽位是注册时收取的 Arc 强引用，句柄在界内即该
    // Scope 由表持有、绝不释放；Arc 堆块地址稳定，`Arc::as_ptr` 指向对齐、
    // 完整的 `Scope`。返回借用只读，分析会话单线程驱动（契约 2），与表
    // 借用不重叠。
    unsafe { reg.borrow().resolve(id.0) }
  })
}

/// 解析点（写回出口）：句柄 → scope 可变视图。空哨兵与越界句柄返回 `None`。
///
/// # 可变借用纪律（对应迁移前的 `arc_as_mut` 写回前提）
/// 同一句柄至多存在一个存活 `&mut`：scope 树的构建与检查全程单分析线程
/// 串行驱动，写回点（父 scope 的 `type_alias_type_parameters`/
/// `type_alias_type_pack_parameters` 缓存、`children` 追加、interior-free
/// 收集）顺序借用、互不重叠——与原先经 `arc_as_mut` 重建 `&mut Scope` 的
/// 别名形态逐位同构（见模块契约 1、2）。
pub(crate) fn resolve_scope_mut<'a>(id: ScopeId) -> Option<&'a mut Scope> {
  if id.is_null() {
    return None;
  }
  SCOPE_REGISTRY.with(|reg| {
    // SAFETY: 契约 1——表内槽位是注册时收取的 Arc 强引用，句柄在界内即该
    // Scope 由表持有、绝不释放，Arc 堆块地址稳定（const→mut 还原与
    // `arc_as_mut` 对同一地址重建 `&mut` 同前提）；契约 2——单分析线程且
    // 调用点遵守上面的可变借用纪律，同一时刻至多一个并存 `&mut`，无数据
    // 竞争。
    unsafe { reg.borrow().resolve_mut(id.0) }
  })
}

// 编译期契约：children/parent 句柄化后 `Scope` 不再含任何 `*mut Scope`/
// `Arc<Scope>` 树导航裸指针；但 `TypeId`/`TypePackId` 字段仍为 `*const Type`/
// `*const TypePackVar` 类型 arena 裸指针（类型 arena 句柄化属后续任务），
// `Scope` 的 `unsafe impl Send/Sync` 因此在本次改造后仍须保留（见
// `records/scope.rs` 文末），待该任务落地后方可移除。
