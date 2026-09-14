use ulua_ast::records::ast_expr_binary::AstExprBinary;

use crate::{
  records::constraint_generator::ConstraintGenerator,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_scope_ptr_ast_expr_binary_optional_type_id(
    &mut self,
    scope: &ScopePtr,
    binary: *mut AstExprBinary,
    expected_type: Option<TypeId>,
  ) -> TypeId {
    let location = unsafe { &(*binary).base.base.location };

    let inference = self.check_ast_expr_binary(
      scope,
      *location,
      unsafe { (*binary).op },
      unsafe { (*binary).left },
      unsafe { (*binary).right },
      expected_type,
    );

    inference.ty
  }
}
