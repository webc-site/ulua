use ulua_common::macros::luau_noinline::LUAU_NOINLINE;

use crate::records::{ast_expr::AstExpr, location::Location, parser::Parser};

impl Parser {
  LUAU_NOINLINE! {
      pub(crate) fn report_function_args_error(&mut self, func: *mut AstExpr, self_flag: bool) -> *mut AstExpr {
          let current_lexeme = self.lexer.current();
          let func_loc = unsafe {
            // Safety: func 为调用方（parse 函数调用/方法调用路径）刚 arena 分配的存活表达式节点，alloc_expr 恒非空（失败 handle_alloc_error 中止）；(*func).base.location 是 #[repr(C)] 基类前缀字段的只读拷贝，单线程串行、读后即返回，无 &mut 别名。
            (*func).base.location
          };

          if self_flag && current_lexeme.location.begin.line != func_loc.end.line {
              let expressions = self.copy_initializer_list_t(&[func]);
              self.report_expr_error(
                  func_loc,
                  expressions,
                  format_args!("Expected function call arguments after '('"),
              )
          } else {
              // Read everything off `current_lexeme` (which borrows self.lexer)
              // BEFORE the `&mut self` calls below, or the borrow conflicts.
              let loc = Location::new(func_loc.begin, current_lexeme.location.begin);
              let lexeme_str = current_lexeme.to_string();
              let expressions = self.copy_initializer_list_t(&[func]);
              self.report_expr_error(
                  loc,
                  expressions,
                  format_args!(
                      "Expected '(', '{{' or <string> when parsing function call, got {}",
                      lexeme_str
                  ),
              )
          }
      }
  }
}
