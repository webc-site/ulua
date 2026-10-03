use crate::{
  records::{ast_type_union::AstTypeUnion, ast_visitor::AstVisitor, node_handle::OptNode},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit_ref},
};

impl_visitable!(AstTypeUnion, TypeUnion, |this, visitor| {
  // types 元素由 union 解析逐个分配进 arena；null 折叠跳过（等价旧 dispatch
  // 短路），解引用经句柄边界，调用点无 unsafe。
  for &type_ptr in this.types.iter() {
    if let Some(ty) = OptNode::from_ptr(type_ptr).get_mut() {
      ast_type_visit_ref(ty, visitor);
    }
  }
});
