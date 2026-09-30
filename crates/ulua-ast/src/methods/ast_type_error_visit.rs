use crate::{
  records::{ast_type_error::AstTypeError, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit},
};

impl_visitable!(AstTypeError, TypeError, |this, visitor| {
  for type_ptr in this.types.iter() {
    // Safety: types 元素是错误类型节点构造时从 TempVector 复制进 arena 的 AstType 槽位；null 被 dispatch 短路，存活节点地址稳定。
    unsafe {
      ast_type_visit(*type_ptr, visitor);
    }
  }
});
