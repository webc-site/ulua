use crate::{
  enums::type_lexer::Type,
  records::{
    ast_expr::AstExpr, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, location::Location, node_handle::Node, parser::Parser,
  },
};

impl Parser {
  pub fn parse_class_ref_expr(&mut self) -> *mut AstExpr {
    let start = self.lexer.current().location.begin;

    let name = self.parse_name_expr("class reference expression");

    let dot_or_bracket = *self.lexer.current();
    if dot_or_bracket.r#type == Type::DOT {
      self.next_lexeme();
      let dot_position = dot_or_bracket.location.begin;
      let index = self.parse_index_name("class reference expression", &dot_position);

      // name 出自 parse_name_expr 线的 arena 分配（恒非空）。
      self.alloc_expr(AstExprIndexName::new(
        Location::new(start, index.location.end),
        Node::from_raw(name),
        index.name,
        index.location,
        dot_position,
        b'.',
      ))
    } else if dot_or_bracket.r#type == Type::LBRACKET {
      self.next_lexeme();
      let key = self.parse_expr(0);
      self.expect_and_consume_char(']', "class reference expression");
      // name/key 出自 arena 分配（parse_name 与 parse_expr 线，恒非空）。
      self.alloc_expr(AstExprIndexExpr::new(
        Location::new(start, self.lexer.previous_location().end),
        Node::from_raw(name),
        Node::from_raw(key),
      ))
    } else {
      name
    }
  }
}
