use ulua_ast::records::{ast_expr::AstExpr, location::Location};

use crate::{
  enums::value_context::ValueContext, records::type_checker_2::TypeChecker2,
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_name(
    &mut self,
    expr: *mut AstExpr,
    location: Location,
    prop_name: &str,
    context: ValueContext,
    ast_index_expr_ty: TypeId,
  ) {
    self.visit_ast_expr_value_context(expr, ValueContext::RValue);
    // SAFETY: expr 指向 AST arena 节点。
    let inferred = self.lookup_type(unsafe { &*expr });
    let left_type = self.strip_from_nil_and_report(inferred, &location);
    self.check_index_type_from_type(left_type, prop_name, context, location, ast_index_expr_ty);
  }
}
