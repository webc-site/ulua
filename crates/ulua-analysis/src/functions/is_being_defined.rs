use core::ptr::NonNull;

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_local::AstStatLocal, node_handle::OptNode},
  rtti::ast_node_try_as,
};

use crate::records::symbol::Symbol;
pub fn is_being_defined(ancestry: &[*mut AstNode], symbol: &Symbol) -> bool {
  if symbol.local.is_none() {
    return false;
  }

  let mut iter = ancestry.len();
  while iter > 0 {
    iter -= 1;
    let node = ancestry[iter];
    // `node` 取自 `ancestry`——parser arena 保活、地址稳定的 AstNode 裸槽位
    // （或显式 null）：先经句柄门面 `OptNode::from_ptr` 折叠可空性，判型下转
    // 走生命周期正确的 [`ast_node_try_as`]，借用半径由本轮局部句柄供给，
    // 不再锻造假 `'static`。
    let handle = OptNode::from_ptr(node);
    let Some(base) = handle.get() else {
      continue;
    };
    let Some(stat_local) = ast_node_try_as::<AstStatLocal>(base) else {
      continue;
    };
    for var in stat_local.vars.iter() {
      // cpp `var == symbol->local`：纯指针身份比较，`var` 侧 `NonNull::new` 归一后比地址。
      if NonNull::new(*var) == symbol.local {
        return true;
      }
    }
  }

  false
}
