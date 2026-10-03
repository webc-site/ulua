//! `AstStatLocal`/`AstStatForIn` 的 vars 类型注解下钻骨架。
//!
//! cpp 两处逐字相同（`Ast.cpp:795-806` 与 `Ast.cpp:826-840`）：遍历 `vars` 槽、
//! 跳过 null 的 `AstLocal`、再对非 null 的 `annotation` 下钻访问。原先在两个
//! `visit` 门面里各抄一份，收口于此。

use crate::{
  records::{
    ast_array::AstArray, ast_local::AstLocal, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::ast_type_visit_ref,
};

pub(crate) fn visit_vars_annotations<V: AstVisitor + ?Sized>(
  vars: AstArray<*mut AstLocal>,
  visitor: &mut V,
) {
  // annotation 仍是 parser 写入 arena 的裸指针（records 波次）：null 折叠与解引用
  // 统一经 `OptNode` 句柄边界，独占契约见 node_handle 模块头。
  for &var_ptr in vars.iter() {
    if let Some(var) = OptNode::from_ptr(var_ptr).get()
      && let Some(ty) = OptNode::from_ptr(var.annotation).get_mut()
    {
      ast_type_visit_ref(ty, visitor);
    }
  }
}
