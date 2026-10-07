use crate::{
  records::{ast_type_optional::AstTypeOptional, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable},
};

impl AstVisitable for AstTypeOptional {
  fn as_ref_mut(&mut self) -> AstNodeRefMut<'_> {
    AstNodeRefMut::TypeOptional(self)
  }

  // 与 cpp Ast.cpp 的 AstTypeOptional::visit 对齐：只回调本节点。本节点没有内层类型
  // 成员（见 `AstTypeOptional` 的说明），故也没有可递归的子节点，不重写 visit_children。
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    visitor.visit_any(self.as_ref_mut());
  }
}
