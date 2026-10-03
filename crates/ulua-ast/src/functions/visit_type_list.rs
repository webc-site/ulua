//! `visit_type_list` (`Ast/src/Ast.cpp`) — 递归遍历 `AstTypeList`
//! （其元素类型与可选 tail pack）。独占借用入参：子节点写穿视图经句柄边界
//! [`OptNode`] 取得，本模块不再携带 `unsafe`。

use crate::{
  records::{
    ast_type_list::AstTypeList, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{ast_type_pack_visit_ref, ast_type_visit_ref},
};

pub fn visit_type_list<V: AstVisitor + ?Sized>(visitor: &mut V, list: &mut AstTypeList) {
  for &ty in list.types.iter() {
    // Safety(句柄边界): 元素为 parser 写入 arena 的 AstType 槽，null 折叠跳过
    // （等价旧指针门面的 null 短路），存活与独占前提见 node_handle 模块契约。
    if let Some(t) = OptNode::from_ptr(ty).get_mut() {
      ast_type_visit_ref(t, visitor);
    }
  }

  if let Some(pack) = OptNode::from_ptr(list.tail_type).get_mut() {
    ast_type_pack_visit_ref(pack, visitor);
  }
}
