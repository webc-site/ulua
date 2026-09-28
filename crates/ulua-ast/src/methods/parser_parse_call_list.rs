use crate::{
  enums::type_lexer::Type,
  functions::optional_node::slot_ref,
  records::{
    ast_array::AstArray, ast_expr::AstExpr, location::Location, match_lexeme::MatchLexeme,
    parser::Parser, position::Position, temp_vector::TempVector,
  },
};

impl Parser {
  pub(crate) fn parse_call_list(
    &mut self,
    comma_positions: Option<&mut TempVector<'_, Position>>,
  ) -> (AstArray<*mut AstExpr>, Location, Location) {
    let current_type = self.lexer.current().r#type;
    ulua_common::LUAU_ASSERT!(
      current_type == Type::LPAREN
        || current_type == Type::LBRACE
        || current_type == Type::RAW_STRING
        || current_type == Type::QUOTED_STRING
    );

    if current_type == Type::LPAREN {
      let arg_start = self.lexer.current().location.end;

      let match_paren = MatchLexeme::new(self.lexer.current());
      self.next_lexeme();

      let mut args = TempVector::new(&mut self.scratch_expr);

      if self.lexer.current().r#type != Type::RPAREN {
        self.parse_expr_list(&mut args, comma_positions);
      }

      let end = self.lexer.current().location;
      let arg_end = end.end;

      self.expect_match_and_consume(')', &match_paren, false);

      let args_array = self.copy_temp_vector_t(&args);
      (
        args_array,
        Location::new(arg_start, arg_end),
        Location::new(match_paren.position, self.lexer.previous_location().begin),
      )
    } else if current_type == Type::LBRACE {
      let arg_start = self.lexer.current().location.end;
      let expr = self.parse_table_constructor();
      let arg_end = self.lexer.previous_location().end;

      let expr_array = self.copy_initializer_list_t(&[expr]);
      // expr 为刚 arena 分配的存活节点（恒非空，失败中止），基类前缀字段只读拷贝。
      (
        expr_array,
        Location::new(arg_start, arg_end),
        slot_ref(expr).base.location,
      )
    } else {
      let arg_location = self.lexer.current().location;
      let expr = self.parse_string();
      let expr_array = self.copy_initializer_list_t(&[expr]);
      (expr_array, arg_location, slot_ref(expr).base.location)
    }
  }
}
