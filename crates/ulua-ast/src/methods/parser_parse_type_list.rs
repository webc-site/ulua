//! Source: `Ast/src/Parser.cpp:2424`
//!
//! Faithful port of `Parser::parseTypeList` — a comma-separated list of types,
//! optionally with `name:` argument labels (which back-fill empty name slots for
//! earlier unlabelled entries). Returns the trailing pack annotation when the list
//! ends in a `...`/named pack; `None` is cpp's `nullptr`, i.e. "no tail".
//! `result`/`result_names` are filled in place for the caller; the optional comma /
//! name-colon position vectors mirror the C++ `TempVector<Position>*` out-params
//! (`None` == `nullptr`).

use core::ptr::NonNull;

use crate::{
  enums::type_lexer::Type,
  functions::should_parse_type_pack::should_parse_type_pack,
  records::{
    ast_type::AstType, ast_type_pack::AstTypePack, parser::Parser, position::Position,
    temp_vector::TempVector,
  },
  type_aliases::ast_argument_name::AstArgumentName,
};

impl Parser {
  /// cpp `Parser::parseTypeList`（`Parser.cpp:2505`）。
  ///
  /// 返回的是「列表以 `...Pack` 收尾时的尾注」：cpp 在该情形 `return parseTypePack()`，
  /// 其余路径统一 `return nullptr`（`Parser.cpp:2558`），即 `None` == 「无尾注」，
  /// 与「有尾注但类型未知」无关（`AstTypeList::tailType` 的 cpp 注释同此）。
  pub(crate) fn parse_type_list(
    &mut self,
    result: &mut TempVector<'_, *mut AstType>,
    result_names: &mut TempVector<'_, Option<AstArgumentName>>,
    mut comma_positions: Option<&mut TempVector<'_, Position>>,
    mut name_colon_positions: Option<&mut TempVector<'_, Position>>,
  ) -> Option<NonNull<AstTypePack>> {
    loop {
      if should_parse_type_pack(&mut self.lexer) {
        return self.parse_type_pack();
      }

      if self.lexer.current().r#type == Type::NAME && self.lexer.lookahead().r#type == Type::COLON {
        // Fill in previous argument names with empty slots
        while result_names.len() < result.len() {
          result_names.push_back(None);
        }
        if let Some(name_colon_positions) = &mut name_colon_positions {
          while name_colon_positions.len() < result.len() {
            name_colon_positions.push_back(Position::missing());
          }
        }

        let arg_name: AstArgumentName =
          (self.lexer.current().name(), self.lexer.current().location);
        result_names.push_back(Some(arg_name));
        self.next_lexeme();

        if let Some(name_colon_positions) = &mut name_colon_positions {
          let begin = self.lexer.current().location.begin;
          name_colon_positions.push_back(begin);
        }
        self.expect_and_consume_char(':', "");
      } else if !result_names.is_empty() {
        // If we have a type with named arguments, provide elements for all types
        result_names.push_back(None);
        if let Some(name_colon_positions) = &mut name_colon_positions {
          name_colon_positions.push_back(Position::missing());
        }
      }

      let ty = self.parse_type(false);
      result.push_back(ty);
      if self.lexer.current().r#type != Type::COMMA {
        break;
      }

      if let Some(comma_positions) = &mut comma_positions {
        let begin = self.lexer.current().location.begin;
        comma_positions.push_back(begin);
      }
      self.next_lexeme();

      if self.lexer.current().r#type == Type::RPAREN {
        let loc = self.lexer.current().location;
        self.report(
          loc,
          format_args!("Expected type after ',' but got ')' instead"),
        );
        break;
      }
    }

    // cpp `return nullptr`（Parser.cpp:2558）：列表未以 pack 收尾，即「无尾注」。
    None
  }
}
