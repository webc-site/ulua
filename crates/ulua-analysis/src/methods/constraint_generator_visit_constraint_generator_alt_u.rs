use alloc::sync::Arc;

use ulua_ast::records::{ast_stat::AstStat, ast_stat_compound_assign::AstStatCompoundAssign};

use crate::{
  enums::control_flow::ControlFlow,
  records::{constraint_generator::ConstraintGenerator, module::Module},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  // ConstraintGenerator::visit(const ScopePtr&, AstStatCompoundAssign*)
  // (ConstraintGenerator.cpp:2046).
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_compound_assign(
    &mut self,
    scope: &ScopePtr,
    assign: *mut AstStatCompoundAssign,
  ) -> ControlFlow {
    unsafe {
      let assign_ref = &*assign;
      let result_ty: TypeId = self
        .check_ast_expr_binary(
          scope,
          assign_ref.base.base.location,
          assign_ref.op,
          assign_ref.var,
          assign_ref.value,
          None,
        )
        .ty;
      let module_ptr = Arc::as_ptr(self.module.as_ref().unwrap()) as *mut Module;
      *(*module_ptr)
        .ast_compound_assign_result_types
        .get_or_insert(assign as *const AstStat) = result_ty;
      // NOTE: We do not update lvalues for compound assignments. This is
      // intentional.
      ControlFlow::None
    }
  }
}
