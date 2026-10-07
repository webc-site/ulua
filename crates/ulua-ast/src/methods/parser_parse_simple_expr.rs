use crate::{
  enums::type_lexer::Type,
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_expr::AstExpr,
    ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_nil::AstExprConstantNil,
    ast_expr_varargs::AstExprVarargs, ast_name::AstName, parser::Parser,
  },
};

impl Parser {
  pub fn parse_simple_expr(&mut self) -> *mut AstExpr {
    let start = self.lexer.current().location;
    let mut ty = self.lexer.current().r#type;

    // `@`/`@[` 前缀：解析属性并要求后随 `function`（对应 C++ parseAttributedFunction）。
    let attributes: AstArray<*mut AstAttr> = if ty == Type::ATTRIBUTE || ty == Type::ATTRIBUTE_OPEN
    {
      let attributes = self.parse_attributes();

      if self.lexer.current().r#type != Type::RESERVED_FUNCTION {
        let current = *self.lexer.current();
        return self.report_expr_error(
          start,
          AstArray::EMPTY,
          format_args!(
            "Expected 'function' declaration after attribute, but got {current} instead",
          ),
        );
      }

      // 属性行结束后 token 已前进，取当前 token 供下方 match 派发。
      ty = self.lexer.current().r#type;
      attributes
    } else {
      AstArray::EMPTY
    };

    // 原 14 段 if-else 链每段都重新取 lexer.current()；改单次求值 + match，
    // 编译器按比较树/跳转表派发。
    match ty {
      Type::RESERVED_NIL => {
        self.next_lexeme();
        self.alloc_expr(AstExprConstantNil::new(start))
      }
      Type::RESERVED_TRUE => {
        self.next_lexeme();
        self.alloc_expr(AstExprConstantBool::new(start, true))
      }
      Type::RESERVED_FALSE => {
        self.next_lexeme();
        self.alloc_expr(AstExprConstantBool::new(start, false))
      }
      Type::RESERVED_FUNCTION => {
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
          .0
          .cast::<AstExpr>()
      }
      Type::NUMBER => self.parse_number(),
      Type::RAW_STRING | Type::QUOTED_STRING | Type::INTERP_STRING_SIMPLE => self.parse_string(),
      Type::INTERP_STRING_BEGIN => self.parse_interp_string(),
      Type::BROKEN_STRING => {
        self.next_lexeme();
        self.report_expr_error(
          start,
          AstArray::EMPTY,
          format_args!("Malformed string; did you forget to finish it?"),
        )
      }
      Type::BROKEN_INTERP_DOUBLE_BRACE => {
        self.next_lexeme();
        self.report_expr_error(
          start,
          AstArray::EMPTY,
          format_args!(
            "Double braces are not permitted within interpolated strings; did you mean '\\{{'?"
          ),
        )
      }
      Type::DOT3 => {
        self.next_lexeme();
        if self.function_stack.last().is_some_and(|f| f.vararg) {
          self.alloc_expr(AstExprVarargs::new(start))
        } else {
          self.report_expr_error(
            start,
            AstArray::EMPTY,
            format_args!("Cannot use '...' outside of a vararg function"),
          )
        }
      }
      Type::LBRACE => {
        // cpp Parser.cpp:4137 `'{'` 臂转 parseTableConstructor（非 `'('`）。
        self.parse_table_constructor()
      }
      Type::RESERVED_IF => self.parse_if_else_expr(),
      _ => self.parse_primary_expr(false),
    }
  }
}
