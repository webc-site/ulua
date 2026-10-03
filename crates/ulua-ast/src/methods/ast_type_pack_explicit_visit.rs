use crate::{
  records::{
    ast_type_pack_explicit::AstTypePackExplicit, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_type_pack_visit_ref, ast_type_visit_ref},
};

impl_visitable!(AstTypePackExplicit, TypePackExplicit, |this, visitor| {
  // type_list.types 元素是 parser 写入 arena 的类型槽，tail_type 可空：
  // null 折叠与解引用统一经 `OptNode` 句柄边界（等价旧 dispatch 短路）。
  for &type_ptr in this.type_list.types.iter() {
    if let Some(ty) = OptNode::from_ptr(type_ptr).get_mut() {
      ast_type_visit_ref(ty, visitor);
    }
  }

  if let Some(pack) = OptNode::from_ptr(this.type_list.tail_type).get_mut() {
    ast_type_pack_visit_ref(pack, visitor);
  }
});
