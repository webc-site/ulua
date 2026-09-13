use ulua_common::FFlag::LuauCstExprGroup;

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_expr::AstExpr, ast_expr_group::AstExprGroup, ast_node::AstNode,
    cst_expr_group::CstExprGroup, cst_node::CstNode, location::Location, match_lexeme::MatchLexeme,
    parser::Parser, position::Position,
  },
};

impl Parser {
  pub fn parse_prefix_expr(&mut self) -> *mut AstExpr {
    if self.lexer.current().r#type == Type('(' as i32) {
      let start = self.lexer.current().location.begin;
      let match_paren = MatchLexeme::new(self.lexer.current());
      self.next_lexeme();

      let expr = self.parse_expr_i32(0);
      let mut end = self.lexer.current().location.end;
      let mut close_paren_found = false;

      if self.lexer.current().r#type != Type(')' as i32) {
        let suggestion = if self.lexer.current().r#type == Type('=' as i32) {
          Some("; did you mean to use '{' when defining a table?")
        } else {
          None
        };

        self.expect_match_and_consume_fail(Type(')' as i32), &match_paren, suggestion);
        end = self.lexer.previous_location().end;
      } else {
        close_paren_found = true;
        self.next_lexeme();
      }

      let expr_group =
        unsafe { (*self.allocator).alloc(AstExprGroup::new(Location::new(start, end), expr)) };

      if LuauCstExprGroup.get() && self.options.store_cst_data {
        let close_pos = if close_paren_found {
          self.lexer.previous_location().begin
        } else {
          Position::missing()
        };
        let cst_node = unsafe { (*self.allocator).alloc(CstExprGroup::new(close_pos)) };
        self
          .cst_node_map
          .try_insert(expr_group as *mut AstNode, cst_node as *mut CstNode);
      }

      expr_group as *mut AstExpr
    } else {
      self.parse_name_expr("expression")
    }
  }
}
