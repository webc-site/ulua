use crate::{
  enums::type_lexer::Type,
  records::{
    ast_stat::AstStat, cst_stat_do::CstStatDo, match_lexeme::MatchLexeme, parser::Parser,
    position::Position,
  },
};

impl Parser {
  pub fn parse_do(&mut self) -> *mut AstStat {
    let start = self.lexer.current().location;

    let match_do = *self.lexer.current();
    self.next_lexeme(); // do

    let stats_start = self.lexer.current().location.begin;

    let body = self.parse_block();

    // Safety: `body` 是 parse_block 刚分配返回的非空 arena `AstStatBlock`（地址稳定、此刻仅本指针可达），
    // 通过它写其自身 location 字段不与其它借用重叠（parser 独占该 arena 节点）。
    unsafe {
      (*body).base.base.location.begin = start.begin;
    }

    let end_location = self.lexer.current().location;
    let has_end =
      self.expect_match_end_and_consume(Type::RESERVED_END, &MatchLexeme::new(&match_do));
    // Safety: 同上，`body` 仍指向本 parser 独占的存活 arena 节点，写入其 `has_end` 字段合法。
    unsafe {
      (*body).has_end = has_end;
    }
    if has_end {
      // Safety: 同上，`body` 为存活 arena 节点且无并发借用，更新其结束 location 合法。
      unsafe {
        (*body).base.base.location.end = end_location.end;
      }
    }

    if self.options.store_cst_data {
      let end_position = if has_end {
        end_location.begin
      } else {
        Position::missing()
      };
      self.attach_cst(body, |alloc| {
        alloc.alloc(CstStatDo::new(stats_start, end_position))
      });
    }

    body.cast::<AstStat>()
  }
}
