use ulua_ast::records::ast_expr_instantiate::AstExprInstantiate;
use ulua_common::FFlag;

use crate::{enums::value_context::ValueContext, records::type_checker_2::TypeChecker2};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr_instantiate(
    &mut self,
    explicit_type_instantiation: *mut AstExprInstantiate,
  ) {
    unsafe {
      let expr = (*explicit_type_instantiation).expr;
      self.visit_ast_expr_value_context(expr, ValueContext::RValue);
      if FFlag::LuauExplicitTypeInstantiationSupport.get() {
        // SAFETY: expr 指向 AST arena 节点。
        let fn_ty = self.lookup_type(&*expr);
        let location = (*explicit_type_instantiation).base.base.location;
        let type_arguments = (*explicit_type_instantiation).type_arguments;
        // SAFETY: expr 指向 AST arena 节点。
        self.check_type_instantiation(&*expr, fn_ty, &location, type_arguments);
      }
    }
  }
}
