use ulua_ast::records::ast_stat_while::AstStatWhile;

use crate::{enums::value_context::ValueContext, records::type_checker_2::TypeChecker2};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `while_statement` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_while(&mut self, while_statement: *mut AstStatWhile) {
    unsafe {
      let while_statement = &*while_statement;
      self.visit_ast_expr_value_context(while_statement.condition, ValueContext::RValue);
      self.visit_ast_stat_block(while_statement.body);
    }
  }
}
