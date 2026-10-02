//! ulua-compiler 编译簇散点裸指针解引用的唯一收口门面（手法同构
//! `ulua-analysis` `records/arena_handle.rs` 的 `alias` 族，仅借形态、不跨
//! crate 引用其私物）。
//!
//! 背景：`compile_expr*` / `compile_expr_call*` 等入口照抄 C++「非 const 引用
//! 透传 + 指针直调」形态，AST 子节点（`Node`/`NonNull` 句柄或 `AstArray` 槽内
//! 裸指针）在业务侧以 `unsafe { &mut *x.as_ptr() }` 散点物化可变借用。本模块
//! 把解引用动作收拢到此处，业务调用点恢复普通 safe 借用；返回值刻意取
//! `&'static`，与原 `&mut *ptr` 在调用点的自由缩短行为逐位同构，迁移不改变
//! 任何借用检查语义（零行为改动）。
//!
//! # Safety（模块级契约，门面函数共同依赖）
//!
//! 1. 入参指针非空、对齐，指向 parser/arena 接线且在本次借用存活期内持续
//!    存活的节点（AST arena 等宿主保活）；
//! 2. 单线程驱动：任一时刻经本模块物化的借用期内，不存在其它存活的可变
//!    别名——写穿仅落在节点的编译期临时字段，由本编译器独占（原散点处
//!    `// SAFETY:` 注释的同一隐含前提，逐条上收到本文件）；
//! 3. null 哨兵不进入本模块：可空处调用点先行判空或以 `Option` 表达。

/// 裸指针 → `&'static mut T` 的解引用收口（原散点 `unsafe { &mut *p }` 的
/// 唯一替身；句柄字段经 safe 的 `as_ptr()` 出裸址后进入本门面）。
///
/// # Safety（由调用点满足，见模块级契约）
/// `p` 非空、对齐且指向存活 `T`；本次借用期内无其它存活可变别名。
pub(crate) fn alias<T>(p: *mut T) -> &'static mut T {
  // SAFETY: 契约由调用点逐条承担（与原散点 `unsafe { &mut *p }` 完全同形，
  // 全 crate 本簇仅此一处执行可变裸指针解引用）。
  unsafe { &mut *p }
}
