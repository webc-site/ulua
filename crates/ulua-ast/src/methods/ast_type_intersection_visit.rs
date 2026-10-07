use crate::{
  records::{
    ast_type_intersection::AstTypeIntersection, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit_ref},
};

impl_visitable!(AstTypeIntersection, TypeIntersection, |this, visitor| {
  // types 元素由交集类型解析逐个以 arena 节点填充；null 折叠跳过（等价旧
  // dispatch 短路），解引用经句柄边界，调用点无 unsafe。
  for &type_ptr in this.types.iter() {
    if let Some(ty) = OptNode::from_ptr(type_ptr).get_mut() {
      ast_type_visit_ref(ty, visitor);
    }
  }
});
