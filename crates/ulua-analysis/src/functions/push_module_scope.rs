//! `NonStrictTypeChecker::push_stack` 与 `TypeChecker2::push_stack` 的共享实现
//! （cpp `TypeChecker2.cpp:476` 与旧求解器同款 `pushStack`）：两检查器的
//! `module: *mut Module` + `stack: Vec<Handle<Scope>>` 字段同型同义，逐字相同
//! 的查表-压栈流程收口于此，各自 `impl` 只留一行委托。

use alloc::vec::Vec;

use ulua_ast::records::ast_node::AstNode;

use crate::records::{
  arena_handle::Handle, module::Module, scope::Scope, stack_pusher::StackPusher,
};

/// 按节点已登记的作用域压栈：`module.ast_scopes` 命中则返回守卫，未登记/无模块
/// 返回 `None`。`stack` 是调用检查器独占的作用域栈字段。
///
/// # Safety 契约说明
/// 上方 `module.is_null()` 守卫确保解引用目标非 null；经裸指针得到的 `&Module`
/// 不借用检查器，与 `&mut stack` 分属不同分配，单线程内无别名冲突。
/// `StackPusher::new` 的 unsafe 契约由命中值保证：`scope_ptr` 是 `ast_scopes`
/// 登记的 arena 存活 `*mut Scope`（null 属登记契约违例即 panic）。
pub(crate) fn push_module_scope(
  module: *mut Module,
  stack: &mut Vec<Handle<Scope>>,
  node: *mut AstNode,
) -> Option<StackPusher> {
  if module.is_null() {
    return None;
  }

  // module.ast_scopes: DenseHashMap<*const AstNode, *mut Scope>
  // C++ lookup: module->astScopes.find(node)
  // Safety: 见函数级契约——module 非 null 且遍历期间存活。
  let module = unsafe { &*module };
  module
    .ast_scopes
    .find(&(node as *const AstNode))
    // Safety: scope_ptr 为登记的 arena 存活句柄；stack 由调用方独占。
    .map(|scope_ptr| unsafe { StackPusher::new(stack, Handle::from_ptr(*scope_ptr)) })
}
