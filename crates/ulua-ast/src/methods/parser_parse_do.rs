use crate::{
  enums::type_lexer::Type,
  records::{
    ast_stat::AstStat, cst_stat_do::CstStatDo, match_lexeme::MatchLexeme, node_handle::Node,
    parser::Parser, position::Position,
  },
};

impl Parser {
  pub fn parse_do(&mut self) -> *mut AstStat {
    let start = self.lexer.current().location;

    let match_do = *self.lexer.current();
    self.next_lexeme(); // do

    let stats_start = self.lexer.current().location.begin;

    // body 收进 arena 句柄：parse_block 产出恒非空（alloc 恒非空，失败中止），
    // 下方三处字段写穿经 DerefMut 安全完成（独占性由 parser 对 arena 的独占
    // 与借用检查共同保证）。
    let mut body = Node::from_raw(self.parse_block());

    body.base.base.location.begin = start.begin;

    let end_location = self.lexer.current().location;
    let has_end =
      self.expect_match_end_and_consume(Type::RESERVED_END, &MatchLexeme::new(&match_do));
    body.has_end = has_end;
    if has_end {
      body.base.base.location.end = end_location.end;
    }

    if self.options.store_cst_data {
      let end_position = if has_end {
        end_location.begin
      } else {
        Position::missing()
      };
      self.attach_cst(body.as_ptr(), |alloc| {
        alloc.alloc(CstStatDo::new(stats_start, end_position))
      });
    }

    body.cast::<AstStat>().as_ptr()
  }
}
