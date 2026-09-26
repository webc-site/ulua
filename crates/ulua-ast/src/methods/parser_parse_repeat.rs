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
    let body = self.with_loop_depth(|p| p.parse_block_no_scope());

    let has_until =
      self.expect_match_end_and_consume(Type::RESERVED_UNTIL, &MatchLexeme::new(&match_repeat));
    // Safety: `body` 由 parse_block_no_scope 返回、指向 arena 存活的 `AstStatBlock`（非空、此刻仅本指针可达），
    // 写入其 `has_end` 字段不与其它借用重叠。
    unsafe {
      (*body).has_end = has_until;
    }
    let until_position = if has_until {
      self.lexer.previous_location().begin
    } else {
      Position::missing()
    };

    let cond = self.parse_expr(0);

    self.restore_locals(locals_begin);

    let node = self.alloc_stat(AstStatRepeat::new(
      // Safety: `cond` 由 parse_expr 返回、parser 保证非空的 arena 存活 `AstExpr`；仅读基类 location.end。
      Location::new(start.begin, unsafe { (*cond).base.location.end }),
      // 槽位收进 arena 句柄：cond/body 皆出自 alloc(恒非空)门面（论证见
      // records::ast_stat_repeat 字段注释）。
      Node::from_raw(cond),
      Node::from_raw(body),
      has_until,
    ));

    self.attach_cst(node, |alloc| {
      alloc.alloc(CstStatRepeat::new(until_position))
    });

    node
  }
}
