use crate::{
  enums::type_lexer::Type,
  functions::optional_node::slot_ref,
  records::{
    ast_expr::AstExpr, ast_expr_type_assertion::AstExprTypeAssertion,
    cst_expr_type_assertion::CstExprTypeAssertion, location::Location, node_handle::Node,
    parser::Parser,
  },
};

impl Parser {
  pub fn parse_assertion_expr(&mut self) -> *mut AstExpr {
    let start = self.lexer.current().location;
    let expr = self.parse_simple_expr();

    if self.lexer.current().r#type == Type::DOUBLE_COLON {
      let op_position = self.lexer.current().location.begin;
      self.next_lexeme();
      let annotation = self.parse_type(false);

      let node = self.alloc_expr(AstExprTypeAssertion::new(
        // annotation 为 parse_type 刚在 arena 分配的存活类型节点（恒非空，失败中止；
        // 错误分支亦返回 AstTypeError 节点），仅读 end 坐标拷贝。
        Location::new(start.begin, slot_ref(annotation).base.location.end),
        // expr/annotation 均出自 parse 线的 arena 分配（恒非空）。
        Node::from_raw(expr),
        Node::from_raw(annotation),
      ));

      self.attach_cst(node, |alloc| {
        alloc.alloc(CstExprTypeAssertion::new(op_position))
      });

      node
    } else {
      expr
    }
  }
}
