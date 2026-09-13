use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  enums::type_context::TypeContext,
  functions::checkpoint::checkpoint,
  records::{
    constraint_generator::ConstraintGenerator, in_conditional_context::InConditionalContext,
    inference_pack::InferencePack,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_pack_scope_ptr_ast_expr_call(
    &mut self,
    scope: &ScopePtr,
    call: *mut AstExprCall,
  ) -> InferencePack {
    let func_begin = unsafe { checkpoint(self) };
    let fn_type = {
      let _in_context =
        unsafe { InConditionalContext::new(&mut self.type_context, TypeContext::Default) };
      self
        .check_scope_ptr_ast_expr(scope, unsafe { (*call).func })
        .ty
    };
    let func_end = unsafe { checkpoint(self) };
    unsafe { self.check_expr_call(scope, call, fn_type, func_begin, func_end) }
  }
}
