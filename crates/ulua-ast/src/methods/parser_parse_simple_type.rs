//! Node: `cxx:Method:Luau.Ast:Ast/src/Parser.cpp:3184:parseSimpleType`
//!
//! Faithful port of `Parser::parseSimpleType` — the type-atom dispatch for
//! all simple (non-suffixed) type annotations: `nil`, `true`, `false`, string
//! singletons, interpolated-string errors, `typeof`, qualified names (`a.b`),
//! a (possibly generic) NAME reference such as `A` / `B<number>`, `{ table }`,
//! `( function )`, and `function`. Generic arguments recurse back through
//! `parse_type_params` -> `parse_type` -> `parse_simple_type`, so nested
//! references parse too. CST positions for reference parameters, `typeof`
//! parens, and string-quote details are recorded only under `store_cst_data`.

use core::ptr::null_mut;

use crate::{
  enums::quote_style_cst::QuoteStyle::QuotedDouble,
  records::{
    ast_array::AstArray, ast_name::AstName, ast_node::AstNode, ast_type::AstType,
    ast_type_or_pack::AstTypeOrPack, ast_type_reference::AstTypeReference,
    ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_typeof::AstTypeTypeof,
    cst_node::CstNode, cst_type_reference::CstTypeReference,
    cst_type_singleton_string::CstTypeSingletonString, cst_type_typeof::CstTypeTypeof,
    lexeme::Type, location::Location, match_lexeme::MatchLexeme, parser::Parser,
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

    if self.lexer.current().r#type == Type::ATTRIBUTE
      || self.lexer.current().r#type == Type::ATTRIBUTE_OPEN
    {
      if !in_declaration_context {
        return AstTypeOrPack {
          r#type: self.report_type_error(
            start,
            AstArray {
              data: null_mut(),
              size: 0,
            },
            format_args!("attributes are not allowed in declaration context"),
          ) as *mut AstType,
          type_pack: null_mut(),
        };
      } else {
        let attributes = Parser::parse_attributes(self);
        return self.parse_function_type(allow_pack, &attributes);
      }
    } else if self.lexer.current().r#type == Type::RESERVED_NIL {
      self.next_lexeme();
      let node = unsafe {
        (*self.allocator).alloc(AstTypeReference::new(
          start,
          None,
          self.name_nil,
          None,
          start,
          false,
          AstArray {
            data: null_mut(),
            size: 0,
          },
        )) as *mut AstType
      };
      return AstTypeOrPack {
        r#type: node,
        type_pack: null_mut(),
      };
    } else if self.lexer.current().r#type == Type::RESERVED_TRUE {
      self.next_lexeme();
      return AstTypeOrPack {
        r#type: unsafe {
          (*self.allocator).alloc(AstTypeSingletonBool::new(start, true)) as *mut AstType
        },
        type_pack: null_mut(),
      };
    } else if self.lexer.current().r#type == Type::RESERVED_FALSE {
      self.next_lexeme();
      return AstTypeOrPack {
        r#type: unsafe {
          (*self.allocator).alloc(AstTypeSingletonBool::new(start, false)) as *mut AstType
        },
        type_pack: null_mut(),
      };
    } else if self.lexer.current().r#type == Type::RAW_STRING
      || self.lexer.current().r#type == Type::QUOTED_STRING
    {
      let mut style = QuotedDouble;
      let mut block_depth: u32 = 0;
      if self.options.store_cst_data {
        let (s, bd) = self.extract_string_details();
        style = s;
        block_depth = bd;
      }

      let mut original_string = AstArray {
        data: null_mut(),
        size: 0,
      };

      if let Some(value) = self.parse_char_array(if self.options.store_cst_data {
        Some(&mut original_string)
      } else {
        None
      }) {
        let node = unsafe {
          (*self.allocator).alloc(AstTypeSingletonString::new(start, value)) as *mut AstType
        };
        if self.options.store_cst_data {
          let cst_node = unsafe {
            (*self.allocator).alloc(CstTypeSingletonString::new(
              original_string,
              style,
              block_depth,
            ))
          };
          self
            .cst_node_map
            .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
        }
        return AstTypeOrPack {
          r#type: node,
          type_pack: null_mut(),
        };
      } else {
        return AstTypeOrPack {
          r#type: self.report_type_error(
            start,
            AstArray {
              data: null_mut(),
              size: 0,
            },
            format_args!("String literal contains malformed escape sequence"),
          ) as *mut AstType,
          type_pack: null_mut(),
        };
      }
    } else if self.lexer.current().r#type == Type::INTERP_STRING_BEGIN
      || self.lexer.current().r#type == Type::INTERP_STRING_SIMPLE
    {
      self.parse_interp_string();
      return AstTypeOrPack {
        r#type: self.report_type_error(
          start,
          AstArray {
            data: null_mut(),
            size: 0,
          },
          format_args!("Interpolated string literals cannot be used as types"),
        ) as *mut AstType,
        type_pack: null_mut(),
      };
    } else if self.lexer.current().r#type == Type::BROKEN_STRING {
      self.next_lexeme();
      return AstTypeOrPack {
        r#type: self.report_type_error(
          start,
          AstArray {
            data: null_mut(),
            size: 0,
          },
          format_args!("Malformed string; did you forget to finish it?"),
        ) as *mut AstType,
        type_pack: null_mut(),
      };
    } else if self.lexer.current().r#type == Type::NAME {
      let mut prefix: Option<AstName> = None;
      let mut prefix_point_position = Position::missing();
      let mut prefix_location: Option<Location> = None;
      let mut name = self.parse_name("type name");

      if self.lexer.current().r#type == Type(b'.' as i32) {
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
      } else if name.name.operator_eq_c_char(c"typeof") {
        let typeof_begin = *self.lexer.current();
        let open_paren_found = self.expect_and_consume_char('(', "typeof type");

        let expr = self.parse_expr_i32(0);

        let end = self.lexer.current().location;
        let close_paren_found =
          self.expect_match_and_consume(')', &MatchLexeme::new(&typeof_begin), false);

        let node = unsafe {
          (*self.allocator).alloc(AstTypeTypeof::new(
            Location::new(start.begin, end.end),
            expr,
          )) as *mut AstType
        };
        if self.options.store_cst_data {
          let cst_node = unsafe {
            (*self.allocator).alloc(CstTypeTypeof::new(
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
          };
          self
            .cst_node_map
            .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
        }
        return AstTypeOrPack {
          r#type: node,
          type_pack: null_mut(),
        };
      }

      let mut has_parameters = false;
      let mut parameters = AstArray {
        data: null_mut(),
        size: 0,
      };
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

      let node = unsafe {
        (*self.allocator).alloc(AstTypeReference::new(
          Location::new(start.begin, end.end),
          prefix,
          name.name,
          prefix_location,
          name.location,
          has_parameters,
          parameters,
        )) as *mut AstType
      };
      if self.options.store_cst_data {
        let cst_node = unsafe {
          (*self.allocator).alloc(CstTypeReference::new(
            prefix_point_position,
            parameters_opening_position,
            self.copy_temp_vector_t(&parameters_comma_positions),
            parameters_closing_position,
          ))
        };
        self
          .cst_node_map
          .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
      }
      return AstTypeOrPack {
        r#type: node,
        type_pack: null_mut(),
      };
    } else if self.lexer.current().r#type == Type(b'{' as i32) {
      return AstTypeOrPack {
        r#type: self.parse_table_type(in_declaration_context),
        type_pack: null_mut(),
      };
    } else if self.lexer.current().r#type == Type('(' as i32)
      || self.lexer.current().r#type == Type::LESS
    {
      return self.parse_function_type(
        allow_pack,
        &AstArray {
          data: null_mut(),
          size: 0,
        },
      );
    } else if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
      self.next_lexeme();

      return AstTypeOrPack {
                r#type: self.report_type_error(
                    start,
                    AstArray {
                        data: null_mut(),
                        size: 0,
                    },
                    format_args!(
                        "Using 'function' as a type annotation is not supported, consider replacing with a function type annotation e.g. '(...any) -> ...any'"
                    ),
                ) as *mut AstType,
                type_pack: null_mut(),
            };
    }

    // For a missing type annotation, capture 'space' between last token and the next one
    let ast_error_location = Location::new(self.lexer.previous_location().end, start.begin);
    // The parse error includes the next lexeme to make it easier to display where the error is.
    let parse_error_location = Location::new(self.lexer.previous_location().end, start.end);
    let current = *self.lexer.current();
    AstTypeOrPack {
      r#type: self.report_missing_type_error(
        parse_error_location,
        ast_error_location,
        format_args!("Expected type, got {current}"),
      ) as *mut AstType,
      type_pack: null_mut(),
    }
  }
}
