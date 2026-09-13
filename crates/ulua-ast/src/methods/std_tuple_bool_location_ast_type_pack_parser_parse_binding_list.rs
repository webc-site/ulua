use std::ptr::null_mut;

use crate::records::{
  ast_array::AstArray, ast_type_pack::AstTypePack, binding::Binding, lexeme::Type,
  location::Location, parser::Parser, position::Position, temp_vector::TempVector,
};

impl Parser {
  pub(crate) fn parse_binding_list(
    &mut self,
    result: &mut TempVector<'_, Binding>,
    allow_dot_3: bool,
    comma_positions: *mut AstArray<Position>,
    initial_comma_position: *mut Position,
    vararg_annotation_colon_position: *mut Position,
    is_const: bool,
  ) -> (bool, Location, *mut AstTypePack) {
    let mut local_comma_positions = TempVector::new(&mut self.scratch_position);

    if !comma_positions.is_null() && !initial_comma_position.is_null() {
      unsafe {
        local_comma_positions.push_back(*initial_comma_position);
      }
    }

    loop {
      if self.lexer.current().r#type == Type::DOT3 && allow_dot_3 {
        let vararg_location = self.lexer.current().location;
        self.next_lexeme();

        let mut tail_annotation: *mut AstTypePack = null_mut();
        if self.lexer.current().r#type == Type::COLON {
          if !vararg_annotation_colon_position.is_null() {
            unsafe {
              *vararg_annotation_colon_position = self.lexer.current().location.begin;
            }
          }

          self.next_lexeme();
          tail_annotation = self.parse_variadic_argument_type_pack();
        }

        if !comma_positions.is_null() {
          unsafe {
            *comma_positions = self.copy_temp_vector_t(&local_comma_positions);
          }
        }

        return (true, vararg_location, tail_annotation);
      }

      result.push_back(self.parse_binding(is_const));

      if self.lexer.current().r#type != Type::COMMA {
        break;
      }

      if !comma_positions.is_null() {
        local_comma_positions.push_back(self.lexer.current().location.begin);
      }

      self.next_lexeme();
    }

    if !comma_positions.is_null() {
      unsafe {
        *comma_positions = self.copy_temp_vector_t(&local_comma_positions);
      }
    }

    (false, Location::default(), null_mut())
  }
}
