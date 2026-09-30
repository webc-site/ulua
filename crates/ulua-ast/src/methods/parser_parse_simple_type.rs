//! Source: `Ast/src/Parser.cpp:3184`
//!
//! Faithful port of `Parser::parseSimpleType` — the type-atom dispatch for
//! all simple (non-suffixed) type annotations: `nil`, `true`, `false`, string
//! singletons, interpolated-string errors, `typeof`, qualified names (`a.b`),
//! a (possibly generic) NAME reference such as `A` / `B<number>`, `{ table }`,
//! `( function )`, and `function`. Generic arguments recurse back through
//! `parse_type_params` -> `parse_type` -> `parse_simple_type`, so nested
//! references parse too. CST positions for reference parameters, `typeof`
//! parens, and string-quote details are recorded only under `store_cst_data`.

use crate::{
  enums::{quote_style_cst::QuoteStyle::QuotedDouble, type_lexer::Type},
  records::{
    ast_array::AstArray, ast_name::AstName, ast_type_or_pack::AstTypeOrPack,
    ast_type_reference::AstTypeReference, ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_typeof::AstTypeTypeof,
    cst_type_reference::CstTypeReference, cst_type_singleton_string::CstTypeSingletonString,
    cst_type_typeof::CstTypeTypeof, location::Location, match_lexeme::MatchLexeme, parser::Parser,
    position::Position, temp_vector::TempVector,
  },
};

impl Parser {
  pub fn parse_simple_type(
    &mut self,
    allow_pack: bool,
    in_declaration_context: bool,
  ) -> AstTypeOrPack {
    self.increment_recursion_counter("type annotation");

    let start = self.lexer.current().location;

    // 单值派发：快照当前 token 类型后用 match 跳转表替代 else-if 链。
    // 原链在各条件判断之间只对该值做纯读取、无状态突变（突变仅发生在命中的
    // 分支体内且该分支立即 return），故快照与逐条求值完全等价。
    let ty = self.lexer.current().r#type;

    match ty {
      Type::ATTRIBUTE | Type::ATTRIBUTE_OPEN => {
        if !in_declaration_context {
          return AstTypeOrPack::from_type(self.report_type_error(
            start,
            AstArray::EMPTY,
            format_args!("attributes are not allowed in declaration context"),
          ));
        } else {
          let attributes = self.parse_attributes();
          return self.parse_function_type(allow_pack, &attributes);
        }
      }
      Type::RESERVED_NIL => {
        self.next_lexeme();
        let node = self.alloc_type(AstTypeReference::new(
          start,
          None,
          self.name_nil,
          None,
          start,
          false,
          AstArray::EMPTY,
        ));
        return AstTypeOrPack::from_type(node);
      }
      Type::RESERVED_TRUE => {
        self.next_lexeme();
        return AstTypeOrPack::from_type(self.alloc_type(AstTypeSingletonBool::new(start, true)));
      }
      Type::RESERVED_FALSE => {
        self.next_lexeme();
        return AstTypeOrPack::from_type(self.alloc_type(AstTypeSingletonBool::new(start, false)));
      }
      Type::RAW_STRING | Type::QUOTED_STRING => {
        let mut style = QuotedDouble;
        let mut block_depth: u32 = 0;
        if self.options.store_cst_data {
          let (s, bd) = self.extract_string_details();
          style = s;
          block_depth = bd;
        }

        if let Some((value, original_string)) = self.parse_char_array(self.options.store_cst_data) {
          let node = self.alloc_type(AstTypeSingletonString::new(start, value));
          self.attach_cst(node, |alloc| {
            alloc.alloc(CstTypeSingletonString::new(
              original_string,
              style,
              block_depth,
            ))
          });
          return AstTypeOrPack::from_type(node);
        } else {
          return AstTypeOrPack::from_type(self.report_type_error(
            start,
            AstArray::EMPTY,
            format_args!("String literal contains malformed escape sequence"),
          ));
        }
      }
      Type::INTERP_STRING_BEGIN | Type::INTERP_STRING_SIMPLE => {
        self.parse_interp_string();
        return AstTypeOrPack::from_type(self.report_type_error(
          start,
          AstArray::EMPTY,
          format_args!("Interpolated string literals cannot be used as types"),
        ));
      }
      Type::BROKEN_STRING => {
        self.next_lexeme();
        return AstTypeOrPack::from_type(self.report_type_error(
          start,
          AstArray::EMPTY,
          format_args!("Malformed string; did you forget to finish it?"),
        ));
      }
      Type::NAME => {
        let mut prefix: Option<AstName> = None;
        let mut prefix_point_position = Position::missing();
        let mut prefix_location: Option<Location> = None;
        let mut name = self.parse_name("type name");

        if self.lexer.current().r#type == Type::DOT {
          prefix_point_position = self.lexer.current().location.begin;
          self.next_lexeme();

          prefix = Some(name.name);
          prefix_location = Some(name.location);
          name = self.parse_index_name("field name", &prefix_point_position);
        } else if self.lexer.current().r#type == Type::DOT3 {
          self.report(
            self.lexer.current().location,
            format_args!(
              "Unexpected '...' after type name; type pack is not allowed in this context"
            ),
          );
          self.next_lexeme();
        } else if name.name == "typeof" {
          let typeof_begin = *self.lexer.current();
          let open_paren_found = self.expect_and_consume_char('(', "typeof type");

          let expr = self.parse_expr(0);

          let end = self.lexer.current().location;
          let close_paren_found =
            self.expect_match_and_consume(')', &MatchLexeme::new(&typeof_begin), false);

          let node = self.alloc_type(AstTypeTypeof::new(
            Location::new(start.begin, end.end),
            expr,
          ));
          self.attach_cst(node, |alloc| {
            alloc.alloc(CstTypeTypeof::new(
              if open_paren_found {
                typeof_begin.location.begin
              } else {
                Position::missing()
              },
              if close_paren_found {
                end.begin
              } else {
                Position::missing()
              },
            ))
          });
          return AstTypeOrPack::from_type(node);
        }

        let mut has_parameters = false;
        let mut parameters = AstArray::EMPTY;
        let mut parameters_opening_position = Position::missing();
        let mut parameters_comma_positions = TempVector::new(&mut self.scratch_position);
        let mut parameters_closing_position = Position::missing();

        if self.lexer.current().r#type == Type::LESS {
          has_parameters = true;
          if self.options.store_cst_data {
            parameters = self.parse_type_params(
              Some(&mut parameters_opening_position),
              Some(&mut parameters_comma_positions),
              Some(&mut parameters_closing_position),
            );
          } else {
            parameters = self.parse_type_params(None, None, None);
          }
        }

        let end = *self.lexer.previous_location();

        let node = self.alloc_type(AstTypeReference::new(
          Location::new(start.begin, end.end),
          prefix,
          name.name,
          prefix_location,
          name.location,
          has_parameters,
          parameters,
        ));
        if self.options.store_cst_data {
          let parameters_comma_array = self.copy_temp_vector_t(&parameters_comma_positions);
          self.attach_cst(node, |alloc| {
            alloc.alloc(CstTypeReference::new(
              prefix_point_position,
              parameters_opening_position,
              parameters_comma_array,
              parameters_closing_position,
            ))
          });
        }
        return AstTypeOrPack::from_type(node);
      }
      Type::LBRACE => {
        return AstTypeOrPack::from_type(self.parse_table_type(in_declaration_context));
      }
      Type::LPAREN | Type::LESS => {
        return self.parse_function_type(allow_pack, &AstArray::EMPTY);
      }
      Type::RESERVED_FUNCTION => {
        self.next_lexeme();

        return AstTypeOrPack::from_type(self.report_type_error(
          start,
          AstArray::EMPTY,
          format_args!(
            "Using 'function' as a type annotation is not supported, consider replacing with a function type annotation e.g. '(...any) -> ...any'"
          ),
        ));
      }
      _ => {}
    }

    // For a missing type annotation, capture 'space' between last token and the next one
    let ast_error_location = Location::new(self.lexer.previous_location().end, start.begin);
    // The parse error includes the next lexeme to make it easier to display where the error is.
    let parse_error_location = Location::new(self.lexer.previous_location().end, start.end);
    let current = *self.lexer.current();
    AstTypeOrPack::from_type(self.report_missing_type_error(
      parse_error_location,
      ast_error_location,
      format_args!("Expected type, got {current}"),
    ))
  }
}
