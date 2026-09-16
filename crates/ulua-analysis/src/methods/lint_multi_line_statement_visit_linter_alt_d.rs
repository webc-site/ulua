use ulua_ast::{records::ast_stat_block::AstStatBlock, visit::ast_stat_visit};

use crate::records::{lint_multi_line_statement::LintMultiLineStatement, statement::Statement};
impl LintMultiLineStatement {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_block(&mut self, node: *mut AstStatBlock) -> bool {
    let node_body = unsafe { (*node).body };
    for &stmt in node_body.as_slice() {
      let s = Statement {
        start: unsafe { (*stmt).base.location },
        last_line: unsafe { (*stmt).base.location.begin.line },
        flagged: false,
      };

      self.stack.push(s);

      unsafe {
        ast_stat_visit(stmt, self);
      }

      self.stack.pop();
    }

    false
  }
}
