use ulua_ast::records::{ast_expr::AstExpr, ast_expr_local::AstExprLocal};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::value::Value,
  functions::should_suppress_errors_type_utils::should_suppress_errors,
  records::{
    blocked_type::BlockedType, constraint_generator::ConstraintGenerator,
    normalization_too_complex::NormalizationTooComplex, scope::Scope,
    subtype_constraint::SubtypeConstraint, symbol::Symbol, type_error_data::TypeErrorData,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  // ConstraintGenerator::visitLValue(const ScopePtr&, AstExprLocal*, TypeId)
  // (ConstraintGenerator.cpp).
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_l_value_scope_ptr_ast_expr_local_type_id(
    &mut self,
    scope: &ScopePtr,
    local: *mut AstExprLocal,
    rhs_type: TypeId,
  ) {
    let local_ptr = unsafe { (*local).local };
    let annotated_ty = scope.lookup_symbol(Symbol::from_local(local_ptr));
    LUAU_ASSERT!(annotated_ty.is_some());

    let def_id = unsafe { (*self.dfg).get_def(local as *const AstExprLocal as *const AstExpr) };
    let mut ty = scope.as_ref().lookup_unrefined_type(def_id);

    if let Some(ty_val) = ty {
      if let Some(local_domain) = self.local_types.find_mut(&ty_val)
        && !unsafe { (*local).upvalue }
      {
        local_domain.insert_type_id(rhs_type);
      }
    } else {
      let new_ty = unsafe { (*self.arena).add_type(BlockedType::default()) };
      self
        .local_types
        .get_or_insert(new_ty)
        .insert_type_id(rhs_type);
      ty = Some(new_ty);

      if let Some(annotated_ty_val) = annotated_ty {
        match unsafe { should_suppress_errors(self.normalizer, annotated_ty_val) }.value {
          Value::DoNotSuppress => {}
          Value::Suppress => {
            ty = Some(self.simplify_union(
              scope.clone(),
              unsafe { (*local).base.base.location },
              new_ty,
              unsafe { (*self.builtin_types).error_type },
            ));
          }
          Value::NormalizationFailed => {
            self.report_error(
              unsafe { (*local_ptr).annotation.as_ref().unwrap().base.location },
              TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            );
          }
        }
      }

      unsafe {
        let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
        *(*scope_raw).lvalue_types.get_or_insert(def_id) = ty.unwrap();
      }
    }

    let assigned_ty = ty.unwrap();
    self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, def_id, assigned_ty);
    self.record_inferred_binding(local_ptr, assigned_ty);

    if let Some(annotated_ty_val) = annotated_ty {
      self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        unsafe { (*local).base.base.location },
        ConstraintV::Subtype(SubtypeConstraint {
          sub_type: rhs_type,
          super_type: annotated_ty_val,
        }),
      );
    }
  }
}
