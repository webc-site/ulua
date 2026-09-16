use ulua_ast::records::ast_stat_for::AstStatFor;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `for_statement` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_for(&mut self, for_statement: *mut AstStatFor) -> NonStrictContext {
    let var_annotation = unsafe { (*for_statement).var.as_mut() }.unwrap().annotation;
    self.visit_ast_type(var_annotation);

    let from = unsafe { (*for_statement).from };
    if !from.is_null() {
      self.visit_ast_expr_value_context(from, ValueContext::RValue);
    }

    let to = unsafe { (*for_statement).to };
    if !to.is_null() {
      self.visit_ast_expr_value_context(to, ValueContext::RValue);
    }

    let step = unsafe { (*for_statement).step };
    if !step.is_null() {
      self.visit_ast_expr_value_context(step, ValueContext::RValue);
    }

    let body = unsafe { (*for_statement).body };
    self.visit_ast_stat_block(body)
  }
}
