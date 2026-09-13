use ulua_ast::records::ast_expr::AstExpr;
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning, records::lint_multi_line_statement::LintMultiLineStatement,
};

impl LintMultiLineStatement {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr(&mut self, node: *mut AstExpr) -> bool {
    let node = unsafe { &*node };
    let top = self.stack.last_mut().unwrap();

    if !top.flagged {
      let location = node.base.location;

      if location.begin.line > top.last_line() {
        top.last_line = location.begin.line;

        if location.begin.column <= top.start.begin.column {
          emit_warning(
            unsafe { &mut *self.context },
            Code::MultiLineStatement,
            location,
            format_args!("Statement spans multiple lines; use indentation to silence"),
          );

          top.flagged = true;
        }
      }
    }

    true
  }
}
