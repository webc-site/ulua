use core::ptr::NonNull;

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::node_opt,
  records::{
    ast_array::AstArray, ast_type_pack::AstTypePack, binding::Binding, location::Location,
    parser::Parser, position::Position, temp_vector::TempVector,
  },
};

impl Parser {
  /// cpp `Parser::parseBindingList`（`Parser.cpp:2441`）。
  ///
  /// 元组第三项是 `...` 后的尾注 pack：cpp 无 `...` 时 `nullptr`（`Parser.cpp:2490`），
  /// 故用 `Option<NonNull<AstTypePack>>` 表达，`None` 即「无尾注」。
  pub(crate) fn parse_binding_list(
    &mut self,
    result: &mut TempVector<'_, Binding>,
    allow_dot_3: bool,
    mut comma_positions: Option<&mut AstArray<Position>>,
    initial_comma_position: Option<&Position>,
    mut vararg_annotation_colon_position: Option<&mut Position>,
    is_const: bool,
  ) -> (bool, Location, Option<NonNull<AstTypePack>>) {
    let mut local_comma_positions = TempVector::new(&mut self.scratch_position);

    // out 槽用 Option 表达「调用方是否要记录」，cpp 传 null 关闭记录的形态在 Rust 侧消失。
    if comma_positions.is_some()
      && let Some(initial) = initial_comma_position
    {
      local_comma_positions.push_back(*initial);
    }

    loop {
      if self.lexer.current().r#type == Type::DOT3 && allow_dot_3 {
        let vararg_location = self.lexer.current().location;
        self.next_lexeme();

        let mut tail_annotation: Option<NonNull<AstTypePack>> = None;
        if self.lexer.current().r#type == Type::COLON {
          if let Some(slot) = vararg_annotation_colon_position.as_deref_mut() {
            *slot = self.lexer.current().location.begin;
          }

          self.next_lexeme();
          tail_annotation = node_opt(self.parse_variadic_argument_type_pack());
        }

        if let Some(slot) = comma_positions.as_deref_mut() {
          // copy_temp_vector_t 从 self 的 scratch arena 拷出新数组后整体写入调用方出参槽。
          *slot = self.copy_temp_vector_t(&local_comma_positions);
        }

        return (true, vararg_location, tail_annotation);
      }

      result.push_back(self.parse_binding(is_const));

      if self.lexer.current().r#type != Type::COMMA {
        break;
      }

      if comma_positions.is_some() {
        local_comma_positions.push_back(self.lexer.current().location.begin);
      }

      self.next_lexeme();
    }

    if let Some(slot) = comma_positions {
      // 同上：写入的是新拷贝出的 arena 数组视图，与 self 借用不重叠；此处为出参最后一次使用，直接移出。
      *slot = self.copy_temp_vector_t(&local_comma_positions);
    }

    (false, Location::default(), None)
  }
}
