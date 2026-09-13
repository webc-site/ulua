use core::{ffi::c_char, ptr::null_mut, slice::from_raw_parts};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::brace_type::BraceType::InterpolatedString,
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_interp_string::AstExprInterpString,
    ast_node::AstNode, cst_expr_interp_string::CstExprInterpString, cst_node::CstNode,
    lexeme::Type, lexer::Lexer, location::Location, parser::Parser,
  },
};

impl Parser {
  pub fn parse_interp_string(&mut self) -> *mut AstExpr {
    let mut strings = crate::records::temp_vector::TempVector::new(&mut self.scratch_string);
    let mut source_strings =
      crate::records::temp_vector::TempVector::new(&mut self.scratch_string_2);
    let mut string_positions =
      crate::records::temp_vector::TempVector::new(&mut self.scratch_position);
    let mut expressions = crate::records::temp_vector::TempVector::new(&mut self.scratch_expr);

    let start_location = self.lexer.current().location;
    let mut end_location;

    loop {
      let current_lexeme = *self.lexer.current();
      LUAU_ASSERT!(
        current_lexeme.r#type == Type::INTERP_STRING_BEGIN
          || current_lexeme.r#type == Type::INTERP_STRING_MID
          || current_lexeme.r#type == Type::INTERP_STRING_END
          || current_lexeme.r#type == Type::INTERP_STRING_SIMPLE
      );

      end_location = current_lexeme.location;

      let length = current_lexeme.get_length() as usize;
      let data_ptr = unsafe { current_lexeme.data.data } as *const c_char;
      let bytes = unsafe { from_raw_parts(data_ptr as *const u8, length) };

      if self.options.store_cst_data {
        let source_string = self.copy_bytes(bytes);
        source_strings.push_back(source_string);
        string_positions.push_back(current_lexeme.location.begin);
      }

      let mut data = bytes.to_vec();
      if !Lexer::fixup_quoted_bytes(&mut data) {
        self.next_lexeme();
        return self.report_expr_error(
          Location::new(start_location.begin, end_location.end),
          AstArray {
            data: null_mut(),
            size: 0,
          },
          format_args!("Interpolated string literal contains malformed escape sequence"),
        ) as *mut AstExpr;
      }

      let chars = self.copy_bytes(&data);
      self.next_lexeme();
      strings.push_back(chars);

      if current_lexeme.r#type == Type::INTERP_STRING_END
        || current_lexeme.r#type == Type::INTERP_STRING_SIMPLE
      {
        break;
      }

      let mut error_while_checking = false;

      match self.lexer.current().r#type {
        Type::INTERP_STRING_MID | Type::INTERP_STRING_END => {
          error_while_checking = true;
          self.next_lexeme();
          expressions.push_back(self.report_expr_error(
            end_location,
            AstArray {
              data: null_mut(),
              size: 0,
            },
            format_args!("Malformed interpolated string, expected expression inside '{{}}'"),
          ) as *mut AstExpr);
        }
        Type::BROKEN_STRING => {
          error_while_checking = true;
          self.next_lexeme();
          expressions.push_back(self.report_expr_error(
            end_location,
            AstArray {
              data: null_mut(),
              size: 0,
            },
            format_args!("Malformed interpolated string; did you forget to add a '`'?"),
          ) as *mut AstExpr);
        }
        _ => {
          expressions.push_back(self.parse_expr_i32(0));
        }
      }

      if error_while_checking {
        break;
      }

      match self.lexer.current().r#type {
        Type::INTERP_STRING_BEGIN | Type::INTERP_STRING_MID | Type::INTERP_STRING_END => {}
        Type::BROKEN_INTERP_DOUBLE_BRACE => {
          self.next_lexeme();
          return self.report_expr_error(
            end_location,
            AstArray {
              data: null_mut(),
              size: 0,
            },
            format_args!(
              "Double braces are not permitted within interpolated strings; did you mean '\\{{'?"
            ),
          ) as *mut AstExpr;
        }
        Type::BROKEN_STRING | Type::EOF => {
          if self.lexer.current().r#type == Type::BROKEN_STRING {
            self.next_lexeme();
          }
          let strings_array = self.copy_temp_vector_t(&strings);
          let expressions_array = self.copy_temp_vector_t(&expressions);
          let node = unsafe {
            (*self.allocator).alloc(AstExprInterpString::new(
              Location::new(start_location.begin, self.lexer.previous_location().end),
              strings_array,
              expressions_array,
            ))
          };

          if self.options.store_cst_data {
            let source_strings_array = self.copy_temp_vector_t(&source_strings);
            let string_positions_array = self.copy_temp_vector_t(&string_positions);
            let cst_node = unsafe {
              (*self.allocator).alloc(CstExprInterpString::new(
                source_strings_array,
                string_positions_array,
              ))
            };
            self
              .cst_node_map
              .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
          }

          if let Some(top) = self.lexer.peek_brace_stack_top() {
            if top == InterpolatedString {
              self.report_location_c_char_item(
                *self.lexer.previous_location(),
                format_args!("Malformed interpolated string; did you forget to add a '}}'?"),
              );
            }
          } else {
            self.report_location_c_char_item(
              *self.lexer.previous_location(),
              format_args!("Malformed interpolated string; did you forget to add a '`'?"),
            );
          }

          return node as *mut AstExpr;
        }
        _ => {
          let current = *self.lexer.current();
          return self.report_expr_error(
            end_location,
            AstArray {
              data: null_mut(),
              size: 0,
            },
            format_args!("Malformed interpolated string, got {current}"),
          ) as *mut AstExpr;
        }
      }
    }

    let strings_array = self.copy_temp_vector_t(&strings);
    let expressions_array = self.copy_temp_vector_t(&expressions);
    let node = unsafe {
      (*self.allocator).alloc(AstExprInterpString::new(
        Location::new(start_location.begin, end_location.end),
        strings_array,
        expressions_array,
      ))
    };

    if self.options.store_cst_data {
      let source_strings_array = self.copy_temp_vector_t(&source_strings);
      let string_positions_array = self.copy_temp_vector_t(&string_positions);
      let cst_node = unsafe {
        (*self.allocator).alloc(CstExprInterpString::new(
          source_strings_array,
          string_positions_array,
        ))
      };
      self
        .cst_node_map
        .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
    }

    node as *mut AstExpr
  }
}
