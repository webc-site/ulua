//! Source: `Analysis/src/ConstraintGenerator.cpp:3205-3221` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprGlobal* global)`.
use core::ptr::null_mut;

use ulua_ast::records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{constraint_generator::ConstraintGenerator, inference::Inference},
  type_aliases::{def_id_def::DefId, scope_ptr_type::ScopePtr},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_scope_ptr_ast_expr_global(
    &mut self,
    scope: &ScopePtr,
    global: *mut AstExprGlobal,
  ) -> Inference {
    unsafe {
      let key = (*self.dfg).get_refinement_key(global as *const AstExpr);
      LUAU_ASSERT!(!key.is_null());

      let def = (*key).def as DefId;

      // prepopulateGlobalScope() has already added all global functions to the environment by this point, so any
      // global that is not already in-scope is definitely an unknown symbol.
      if let Some(ty) = self.lookup(
        scope,
        (*global).base.base.location,
        def,
        /*prototype=*/ false,
      ) {
        let refinement = self
          .refinement_arena
          .proposition_refinement_key_type_id(key, (*self.builtin_types).truthy_type);
        Inference::inference_type_id_refinement_id(ty, refinement)
      } else {
        Inference::inference_type_id_refinement_id((*self.builtin_types).error_type, null_mut())
      }
    }
  }
}
