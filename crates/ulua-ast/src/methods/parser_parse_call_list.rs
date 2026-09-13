use crate::records::{
  ast_array::AstArray, ast_expr::AstExpr, lexeme::Type, location::Location,
  match_lexeme::MatchLexeme, parser::Parser, position::Position, temp_vector::TempVector,
};

impl Parser {
  pub(crate) fn parse_call_list(
    &mut self,
    comma_positions: *mut TempVector<'_, Position>,
  ) -> (AstArray<*mut AstExpr>, Location, Location) {
    let current_type = self.lexer.current().r#type;
    ulua_common::LUAU_ASSERT!(
      current_type == Type('(' as i32)
        || current_type == Type('{' as i32)
        || current_type == Type::RAW_STRING
        || current_type == Type::QUOTED_STRING
    );

    if current_type == Type('(' as i32) {
      let arg_start = self.lexer.current().location.end;

      let match_paren = MatchLexeme::new(self.lexer.current());
      self.next_lexeme();

      let mut args = TempVector::new(&mut self.scratch_expr);

      if self.lexer.current().r#type != Type(')' as i32) {
        if !comma_positions.is_null() {
          self.parse_expr_list(&mut args, Some(unsafe { &mut *comma_positions }));
        } else {
          self.parse_expr_list(&mut args, None);
        }
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
    } else if current_type == Type('{' as i32) {
      let arg_start = self.lexer.current().location.end;
      let expr = self.parse_table_constructor();
      let arg_end = self.lexer.previous_location().end;

      let expr_array = self.copy_t_usize(&expr as *const *mut AstExpr, 1);
      (expr_array, Location::new(arg_start, arg_end), unsafe {
        (*expr).base.location
      })
    } else {
      let arg_location = self.lexer.current().location;
      let expr = self.parse_string();
      let expr_array = self.copy_t_usize(&expr as *const *mut AstExpr, 1);
      (expr_array, arg_location, unsafe { (*expr).base.location })
    }
  }
}
