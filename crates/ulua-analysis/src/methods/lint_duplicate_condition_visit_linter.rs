use alloc::vec::Vec;

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_if::AstStatIf},
  rtti::ast_node_as,
  visit::{ast_expr_visit, ast_stat_visit},
};

use crate::records::lint_duplicate_condition::LintDuplicateCondition;

impl LintDuplicateCondition {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_if(&mut self, stat: *mut AstStatIf) -> bool {
    unsafe {
      if stat.is_null() || (*stat).elsebody.is_null() {
        return true;
      }

      if ast_node_as::<AstStatIf>((*stat).elsebody as *mut AstNode).is_null() {
        return true;
      }

      let mut conditions = Vec::with_capacity(2);
      let mut head = stat;

      while !head.is_null() {
        ast_expr_visit((*head).condition, self);
        ast_stat_visit((*head).thenbody as *mut _, self);

        conditions.push((*head).condition);

        if !(*head).elsebody.is_null() {
          let next = ast_node_as::<AstStatIf>((*head).elsebody as *mut AstNode);
          if !next.is_null() {
            head = next;
            continue;
          }

          ast_stat_visit((*head).elsebody, self);
        }

        break;
      }

      self.detect_duplicates(&conditions);
    }

    false
  }
}
