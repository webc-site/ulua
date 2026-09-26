use crate::{
  records::{ast_type_intersection::AstTypeIntersection, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit},
};

impl_visitable!(AstTypeIntersection, TypeIntersection, |this, visitor| {
  for &type_ptr in this.types.iter() {
    // Safety: types 元素由交集类型解析逐个以 arena 节点填充；null 或存活均满足契约，块地址不移动。
    unsafe {
      ast_type_visit(type_ptr, visitor);
    }
  }
});
