use crate::{
  enums::type_lexer::Type,
  records::{
    ast_expr::AstExpr, ast_expr_group::AstExprGroup, cst_expr_group::CstExprGroup,
    location::Location, match_lexeme::MatchLexeme, node_handle::Node, parser::Parser,
    position::Position,
  },
};

impl Parser {
  pub fn parse_prefix_expr(&mut self) -> *mut AstExpr {
    if self.lexer.current().r#type == Type::LPAREN {
      let start = self.lexer.current().location.begin;
      let match_paren = MatchLexeme::new(self.lexer.current());
      self.next_lexeme();

      let expr = self.parse_expr(0);
      let mut end = self.lexer.current().location.end;
      let mut close_paren_found = false;

      if self.lexer.current().r#type != Type::RPAREN {
        let suggestion = if self.lexer.current().r#type == Type::EQUAL_SIGN {
          Some("; did you mean to use '{' when defining a table?")
        } else {
          None
        };

        self.expect_match_and_consume_fail(Type::RPAREN, &match_paren, suggestion);
        end = self.lexer.previous_location().end;
      } else {
        close_paren_found = true;
        self.next_lexeme();
      }

      // expr 出自 parse_expr 的 arena 分配（cpp 语义下错误路径亦返回 AstExprError 节点，恒非空）。
      let expr_group = self.alloc_expr(AstExprGroup::new(
        Location::new(start, end),
        Node::from_raw(expr),
      ));

      if self.options.store_cst_data {
        let close_pos = if close_paren_found {
          self.lexer.previous_location().begin
        } else {
          Position::missing()
        };
        self.attach_cst(expr_group, |alloc| {
          alloc.alloc(CstExprGroup::new(close_pos))
        });
      }

      expr_group
    } else {
      self.parse_name_expr("expression")
    }
  }
}
