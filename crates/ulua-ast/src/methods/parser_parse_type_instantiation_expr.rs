use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  ast_array::AstArray,
  ast_type_or_pack::AstTypeOrPack,
  cst_type_instantiation::CstTypeInstantiation,
  lexeme::{Lexeme, Type},
  location::Location,
  match_lexeme::MatchLexeme,
  parser::Parser,
  position::Position,
  temp_vector::TempVector,
};

impl Parser {
  pub(crate) fn parse_type_instantiation_expr(
    &mut self,
    cst_node_out: *mut CstTypeInstantiation,
    end_location_out: Option<&mut Location>,
  ) -> AstArray<AstTypeOrPack> {
    LUAU_ASSERT!(
      self.lexer.current().r#type == Type::LESS && self.lexer.lookahead().r#type == Type::LESS
    );

    if !cst_node_out.is_null() {
      unsafe {
        (*cst_node_out).left_arrow_1_position = self.lexer.current().location.begin;
      }
    }

    let begin = *self.lexer.current();
    self.next_lexeme();

    let mut comma_positions: TempVector<'_, Position> = TempVector::new(&mut self.scratch_position);

    let type_or_packs = self.parse_type_params(
      if cst_node_out.is_null() {
        None
      } else {
        unsafe { Some(&mut (*cst_node_out).left_arrow_2_position) }
      },
      if cst_node_out.is_null() {
        None
      } else {
        Some(&mut comma_positions)
      },
      if cst_node_out.is_null() {
        None
      } else {
        unsafe { Some(&mut (*cst_node_out).right_arrow_1_position) }
      },
    );

    if !cst_node_out.is_null() {
      unsafe {
        (*cst_node_out).comma_positions = self.copy_temp_vector_t(&comma_positions);
        if self.lexer.current().r#type == Type::GREATER {
          (*cst_node_out).right_arrow_2_position = self.lexer.current().location.begin;
        }
      }
    }

    if let Some(end_location_out) = end_location_out {
      *end_location_out = self.lexer.current().location;
    }

    let begin_match = Lexeme::new(begin.location, begin.r#type);
    self.expect_match_and_consume('>', &MatchLexeme::new(&begin_match), false);

    type_or_packs
  }
}
