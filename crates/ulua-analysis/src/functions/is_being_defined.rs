use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_local::AstStatLocal},
  rtti::ast_node_try_as_ptr,
};

use crate::records::symbol::Symbol;
pub fn is_being_defined(ancestry: &[*mut AstNode], symbol: &Symbol) -> bool {
  if symbol.local.is_null() {
    return false;
  }

  let mut iter = ancestry.len();
  while iter > 0 {
    iter -= 1;
    let node = ancestry[iter];
    // Safety: `node` 取自 `ancestry`——parser arena 保活、地址稳定的 AstNode
    // 句柄（或显式 null），满足 `ast_node_try_as_ptr` 的「null 或存活 repr(C)
    // 节点」契约；RTTI 命中即动态类型为 AstStatLocal，其 `vars` 数组由 parser
    // 成对写入 arena，本次只读遍历期间节点保持存活。
    let Some(stat_local) = (unsafe { ast_node_try_as_ptr::<AstStatLocal>(node) }) else {
      continue;
    };
    for var in stat_local.vars.iter() {
      if *var == symbol.local {
        return true;
      }
    }
  }

  false
}
