use core::ptr::null_mut;

use crate::records::{
  ast_array::AstArray, ast_attr::AstAttr, ast_expr::AstExpr,
  ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_nil::AstExprConstantNil,
  ast_expr_varargs::AstExprVarargs, ast_name::AstName, lexeme::Type, parser::Parser,
};

impl Parser {
  pub fn parse_simple_expr(&mut self) -> *mut AstExpr {
    let start = self.lexer.current().location;

    let mut attributes: AstArray<*mut AstAttr> = AstArray {
      data: null_mut(),
      size: 0,
    };

    if self.lexer.current().r#type == Type::ATTRIBUTE
      || self.lexer.current().r#type == Type::ATTRIBUTE_OPEN
    {
      attributes = self.parse_attributes();

      if self.lexer.current().r#type != Type::RESERVED_FUNCTION {
        let current = *self.lexer.current();
        return self.report_expr_error(
          start,
          AstArray {
            data: null_mut(),
            size: 0,
          },
          format_args!(
            "Expected 'function' declaration after attribute, but got {current} instead",
          ),
        ) as *mut AstExpr;
      }
    }

    if self.lexer.current().r#type == Type::RESERVED_NIL {
      self.next_lexeme();
      unsafe { (*self.allocator).alloc(AstExprConstantNil::new(start)) as *mut AstExpr }
    } else if self.lexer.current().r#type == Type::RESERVED_TRUE {
      self.next_lexeme();
      unsafe { (*self.allocator).alloc(AstExprConstantBool::new(start, true)) as *mut AstExpr }
    } else if self.lexer.current().r#type == Type::RESERVED_FALSE {
      self.next_lexeme();
      unsafe { (*self.allocator).alloc(AstExprConstantBool::new(start, false)) as *mut AstExpr }
    } else if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
      let match_function = *self.lexer.current();
      self.next_lexeme();

      self
        .parse_function_body(
          false,
          &match_function,
          &AstName::new(),
          None,
          &attributes,
          false,
        )
        .0 as *mut AstExpr
    } else if self.lexer.current().r#type == Type::NUMBER {
      self.parse_number()
    } else if self.lexer.current().r#type == Type::RAW_STRING
      || self.lexer.current().r#type == Type::QUOTED_STRING
      || self.lexer.current().r#type == Type::INTERP_STRING_SIMPLE
    {
      self.parse_string()
    } else if self.lexer.current().r#type == Type::INTERP_STRING_BEGIN {
      self.parse_interp_string()
    } else if self.lexer.current().r#type == Type::BROKEN_STRING {
      self.next_lexeme();
      self.report_expr_error(
        start,
        AstArray {
          data: null_mut(),
          size: 0,
        },
        format_args!("Malformed string; did you forget to finish it?"),
      ) as *mut AstExpr
    } else if self.lexer.current().r#type == Type::BROKEN_INTERP_DOUBLE_BRACE {
      self.next_lexeme();
      self.report_expr_error(
        start,
        AstArray {
          data: null_mut(),
          size: 0,
        },
        format_args!(
          "Double braces are not permitted within interpolated strings; did you mean '\\{{'?"
        ),
      ) as *mut AstExpr
    } else if self.lexer.current().r#type == Type::DOT3 {
      if self.function_stack.last().is_some_and(|f| f.vararg) {
        self.next_lexeme();
        unsafe { (*self.allocator).alloc(AstExprVarargs::new(start)) as *mut AstExpr }
      } else {
        self.next_lexeme();
        self.report_expr_error(
          start,
          AstArray {
            data: null_mut(),
            size: 0,
          },
          format_args!("Cannot use '...' outside of a vararg function"),
        ) as *mut AstExpr
      }
    } else if self.lexer.current().r#type == Type('{' as i32) {
      // C++ `else if (lexer.current().type == '{') return parseTableConstructor();`
      // The model mistranslated the brace literal `'{'` as `'('`, routing
      // parenthesized expressions into the table-constructor parser and causing
      // unbounded recursion on any `(expr)`.
      self.parse_table_constructor()
    } else if self.lexer.current().r#type == Type::RESERVED_IF {
      self.parse_if_else_expr()
    } else {
      self.parse_primary_expr(false)
    }
  }
}
