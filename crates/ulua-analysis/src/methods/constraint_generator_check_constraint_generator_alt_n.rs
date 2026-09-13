//! Source: `Analysis/src/ConstraintGenerator.cpp:3550-3570` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprInstantiate* explicitTypeInstantiation)`.
use core::ptr::null_mut;

use ulua_ast::records::ast_expr_instantiate::AstExprInstantiate;
use ulua_common::FFlag;

use crate::{
  functions::get_mutable_type::get_mutable_type_id,
  records::{
    blocked_type::BlockedType, constraint_generator::ConstraintGenerator, inference::Inference,
    scope::Scope, type_instantiation_constraint::TypeInstantiationConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_scope_ptr_ast_expr_instantiate(
    &mut self,
    scope: &ScopePtr,
    explicit_type_instantiation: *mut AstExprInstantiate,
  ) -> Inference {
    unsafe {
      if !FFlag::LuauExplicitTypeInstantiationSupport.get() {
        return self.check_scope_ptr_ast_expr(scope, (*explicit_type_instantiation).expr);
      }

      let function_type = self
        .check_scope_ptr_ast_expr_optional_type_id(scope, (*explicit_type_instantiation).expr, None)
        .ty;

      let (explicit_type_ids, explicit_type_pack_ids) = self.resolve_type_arguments(
        scope.as_ref() as *const Scope as *mut Scope,
        (*explicit_type_instantiation).type_arguments,
      );

      let placeholder_type = (*self.arena).add_type(BlockedType::default());

      let constraint = self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        (*explicit_type_instantiation).base.base.location,
        ConstraintV::TypeInstantiation(TypeInstantiationConstraint {
          function_type,
          placeholder_type,
          type_arguments: explicit_type_ids,
          type_pack_arguments: explicit_type_pack_ids,
        }),
      );

      // placeholder_type 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3630
      // `getMutable<BlockedType>(placeholderType)->setOwner(constraint)`
      let blocked = get_mutable_type_id::<BlockedType>(placeholder_type).unwrap();
      blocked.set_owner(constraint as *const _);

      Inference::inference_type_id_refinement_id(placeholder_type, null_mut())
    }
  }
}
