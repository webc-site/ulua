use crate::{
  enums::type_lexer::Type,
  functions::optional_node::slot_ref,
  records::{
    ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName, ast_name::AstName,
    location::Location, node_handle::Node, parser::Parser,
  },
};

impl Parser {
  pub fn parse_function_name(
    &mut self,
    hasself: &mut bool,
    debugname: &mut AstName,
  ) -> *mut AstExpr {
    let current = self.lexer.current();
    if current.r#type == Type::NAME {
      *debugname = current.name();
    }

    // parse funcname into a chain of indexing operators
    let mut expr = self.parse_name_expr("function name");

    let old_recursion_count = self.recursion_counter;

    while self.lexer.current().r#type == Type::DOT {
      let op_position = self.lexer.current().location.begin;
      self.next_lexeme();

      let name = self.parse_name("field name");

      // while we could concatenate the name chain, for now let's just write the short name
      *debugname = name.name;

      let location = Location::new(slot_ref(expr).base.location.begin, name.location.end);
      // expr 出自 parse_name_expr/上一轮 alloc 分支的 arena 分配（恒非空）。
      expr = self.alloc_expr(AstExprIndexName::new(
        location,
        Node::from_raw(expr),
        name.name,
        name.location,
        op_position,
        b'.',
      ));

      // note: while the parser isn't recursive here, we're generating recursive structures of unbounded depth
      self.increment_recursion_counter("function name");
    }

    self.recursion_counter = old_recursion_count;

    // finish with :
    if self.lexer.current().r#type == Type::COLON {
      let op_position = self.lexer.current().location.begin;
      self.next_lexeme();

      let name = self.parse_name("method name");

      // while we could concatenate the name chain, for now let's just write the short name
      *debugname = name.name;

      let location = Location::new(slot_ref(expr).base.location.begin, name.location.end);
      // expr 出自上一轮 alloc 分支的 arena 分配（恒非空）。
      expr = self.alloc_expr(AstExprIndexName::new(
        location,
        Node::from_raw(expr),
        name.name,
        name.location,
        op_position,
        b':',
      ));

      *hasself = true;
    }

    expr
  }
}
