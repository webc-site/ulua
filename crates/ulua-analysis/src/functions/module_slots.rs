//! `Module`（cpp `Module*`）的 AST→类型登记表写门面（`review.md` §2
//! 「非空指针 → 引用/句柄」在模块信息侧的落点）。
//!
//! cpp 侧 `module->astTypes[expr] = ty` 是对 `DenseHashMap<AstExpr*, TypeId>`
//! 的普通写入；Rust 侧 `self.module` 是 `Arc<Module>`（同一 `Arc` 另存在
//! `Frontend` 侧强引用，故 `Arc` 借不出 `&mut`），照抄写法会在每个写入点拖出
//! 「`arc_as_mut` 派生裸指针 + `(*p).ast_types.get_or_insert(..)` 解引用」。
//! 本门面把这两步合成一次具名写入，visit 侧只交出节点共享借用。
//!
//! # 契约
//!
//! 1. `expr` 由调用方从 parse arena 交出的存活节点借用还原地址（与 cpp 传入
//!    的 `AstExpr*` 同一目标），AST 在检查会话内只读不搬迁，故作为哈希键稳定；
//! 2. `module` 由调用方持有的 `&Arc<Module>` 保活，句柄只在本次写入内存活；
//! 3. 写入只在 `ast_types` 的槽位上发生，本模块不改变任何既有的键/值语义
//!    （`get_or_insert` 与 cpp `operator[]` 的「缺失即插入默认值再赋值」同构）。

use alloc::sync::Arc;
use core::ptr::from_ref;

use ulua_ast::rtti::AstNodeView;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{arena_handle::alias, module::Module},
  type_aliases::{name_type::Name, type_id::TypeId},
};

/// cpp `module->astTypes[expr] = ty`。
///
/// 键取节点基类地址：`repr(C)` 首字段链保证 `&E`、其 `AstNode` 视图与 cpp 的
/// `AstExpr*` 同址（与 `ulua_ast::rtti` 的下转同一布局前提）。
#[inline]
pub(crate) fn record_expr_type<E: AstNodeView>(module: &Arc<Module>, expr: &E, ty: TypeId) {
  *alias(arc_as_mut(module))
    .ast_types
    .get_or_insert(from_ref(expr.as_ast_node()).cast()) = ty;
}

/// cpp `module->astCompoundAssignResultTypes[stat] = ty`（键同上，走 AstStat 视图）。
#[inline]
pub(crate) fn record_stat_type<S: AstNodeView>(module: &Arc<Module>, stat: &S, ty: TypeId) {
  *alias(arc_as_mut(module))
    .ast_compound_assign_result_types
    .get_or_insert(from_ref(stat.as_ast_node()).cast()) = ty;
}

/// cpp `module->declaredGlobals[name] = ty`（`HashMap` 覆盖写，与 cpp `[]=` 同构）。
#[inline]
pub(crate) fn record_declared_global(module: &Arc<Module>, name: Name, ty: TypeId) {
  alias(arc_as_mut(module)).declared_globals.insert(name, ty);
}
