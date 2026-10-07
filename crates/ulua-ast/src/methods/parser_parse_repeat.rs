use crate::{
  enums::type_lexer::Type,
  records::{
    ast_stat::AstStat, ast_stat_repeat::AstStatRepeat, cst_stat_repeat::CstStatRepeat,
    location::Location, match_lexeme::MatchLexeme, node_handle::Node, parser::Parser,
    position::Position,
  },
};

impl Parser {
  pub fn parse_repeat(&mut self) -> *mut AstStat {
    let start = self.lexer.current().location;

    let match_repeat = *self.lexer.current();
    self.next_lexeme(); // repeat

    let locals_begin = self.save_locals();

    // loop_depth 升降为循环骨架，见 parser_loop_body（repeat 直到 until 前，
    // 体与条件同处一层作用域，故用 parse_block_no_scope）
    let mut body = Node::from_raw(self.with_loop_depth(|p| p.parse_block_no_scope()));

    let has_until =
      self.expect_match_end_and_consume(Type::RESERVED_UNTIL, &MatchLexeme::new(&match_repeat));
    body.has_end = has_until;
    let until_position = if has_until {
      self.lexer.previous_location().begin
    } else {
      Position::missing()
    };

    let cond = Node::from_raw(self.parse_expr(0));

    self.restore_locals(locals_begin);

    let node = self.alloc_stat(AstStatRepeat::new(
      // cond/base.location 经句柄 Deref 只读拷贝（arena 存活，恒非空）。
      Location::new(start.begin, cond.base.location.end),
      cond,
      body,
      has_until,
    ));

    self.attach_cst(node, |alloc| {
      alloc.alloc(CstStatRepeat::new(until_position))
    });

    node
  }
}
