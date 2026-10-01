use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_array::AstArray, ast_type_or_pack::AstTypeOrPack,
    cst_type_instantiation::CstTypeInstantiation, lexeme::Lexeme, location::Location,
    match_lexeme::MatchLexeme, parser::Parser, position::Position, temp_vector::TempVector,
  },
};

impl Parser {
  pub(crate) fn parse_type_instantiation_expr(
    &mut self,
    mut cst_node_out: Option<&mut CstTypeInstantiation>,
    end_location_out: Option<&mut Location>,
  ) -> AstArray<AstTypeOrPack> {
    LUAU_ASSERT!(
      self.lexer.current().r#type == Type::LESS && self.lexer.lookahead().r#type == Type::LESS
    );

    // CST 出参槽用 Option 表达「调用方是否要记录」，cpp 传 null 关闭记录的形态在 Rust 侧消失；
    // 全部字段写入经借用完成，原三处裸指针 unsafe 写一并消解。
    if let Some(cst) = &mut cst_node_out {
      cst.left_arrow_1_position = self.lexer.current().location.begin;
    }

    let begin = *self.lexer.current();
    self.next_lexeme();

    let mut comma_positions: TempVector<'_, Position> = TempVector::new(&mut self.scratch_position);

    let type_or_packs = match &mut cst_node_out {
      Some(cst) => {
        // 先做一层局部再借用，随后三个 disjoint 字段借位互不重叠。
        let cst = &mut **cst;
        self.parse_type_params(
          Some(&mut cst.left_arrow_2_position),
          Some(&mut comma_positions),
          Some(&mut cst.right_arrow_1_position),
        )
      }
      None => self.parse_type_params(None, None, None),
    };

    if let Some(cst) = &mut cst_node_out {
      // copy_temp_vector_t 产出的新 arena 数组整体写入出参槽字段，写位与 self 借用不重叠。
      cst.comma_positions = self.copy_temp_vector_t(&comma_positions);
      if self.lexer.current().r#type == Type::GREATER {
        cst.right_arrow_2_position = self.lexer.current().location.begin;
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
