//! Source: `Analysis/src/ConstraintGenerator.cpp:3182-3203` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprLocal* local)`.
use core::ptr::null_mut;

use ulua_ast::records::{ast_expr::AstExpr, ast_expr_local::AstExprLocal};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::follow_type::follow,
  records::{constraint_generator::ConstraintGenerator, inference::Inference},
  type_aliases::{def_id_def::DefId, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_scope_ptr_ast_expr_local(
    &mut self,
    scope: &ScopePtr,
    local: *mut AstExprLocal,
  ) -> Inference {
    unsafe {
      let key = (*self.dfg).get_refinement_key(local as *const AstExpr);
      LUAU_ASSERT!(!key.is_null());

      let mut maybe_ty: Option<TypeId> = None;

      // if we have a refinement key, we can look up its type.
      if !key.is_null() {
        // C++ default `prototype = true`.
        maybe_ty = self.lookup(
          scope,
          (*local).base.base.location,
          (*key).def as DefId,
          true,
        );
      }

      if let Some(ty) = maybe_ty {
        let ty = follow(ty);

        self.record_inferred_binding((*local).local, ty);

        let refinement = self
          .refinement_arena
          .proposition_refinement_key_type_id(key, (*self.builtin_types).truthy_type);
        Inference::inference_type_id_refinement_id(ty, refinement)
      } else {
        (*self.ice).ice_string("CG: AstExprLocal came before its declaration?");
        Inference::inference_type_id_refinement_id((*self.builtin_types).error_type, null_mut())
      }
    }
  }
}
