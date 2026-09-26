use crate::{
  records::{ast_type_union::AstTypeUnion, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit},
};

impl_visitable!(AstTypeUnion, TypeUnion, |this, visitor| {
  for &type_ptr in this.types.iter() {
    // Safety: types 元素由 union 解析逐个分配进 arena；null 短路、存活节点地址不移动，遍历期独占写穿。
    unsafe {
      ast_type_visit(type_ptr, visitor);
    }
  }
});
