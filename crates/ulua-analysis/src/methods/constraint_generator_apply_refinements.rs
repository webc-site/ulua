use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{refinements_op_kind::RefinementsOpKind, value::Value},
  functions::{
    must_defer_intersection::must_defer_intersection,
    should_suppress_errors_type_utils::should_suppress_errors,
  },
  records::{
    constraint_generator::ConstraintGenerator, error_suppression::ErrorSuppression,
    normalization_too_complex::NormalizationTooComplex,
  },
  type_aliases::{
    constraint_v::ConstraintV, refinement_context::RefinementContext,
    refinement_id_refinement::RefinementId, scope_ptr_type::ScopePtr, type_id::TypeId,
  },
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn apply_refinements(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    refinement: RefinementId,
  ) {
    if refinement.is_null() {
      return;
    }

    let mut refinements: RefinementContext = RefinementContext::default();
    let mut constraints: Vec<ConstraintV> = Vec::new();

    unsafe {
      self.compute_refinement(
        scope.as_ref() as *const _ as *mut _,
        location,
        refinement,
        &mut refinements,
        true,
        false,
        &mut constraints,
      )
    };

    let flush_constraints = |this: &mut ConstraintGenerator,
                             kind: RefinementsOpKind,
                             ty: TypeId,
                             discriminants: &mut Vec<TypeId>|
     -> TypeId {
      if discriminants.is_empty() {
        return ty;
      }

      if kind == RefinementsOpKind::None {
        LUAU_ASSERT!(false);
        return ty;
      }

      let mut args = Vec::new();
      args.push(ty);

      let builtin_types = this.builtin_types;
      let type_functions = unsafe { &(*builtin_types).type_functions };

      let func = if kind == RefinementsOpKind::Intersect {
        &type_functions.intersect_func
      } else {
        &type_functions.refine_func
      };

      LUAU_ASSERT!(!func.name.is_empty());
      args.extend_from_slice(discriminants.as_slice());

      let result_type = this.create_type_function_instance(func, args, Vec::new(), scope, location);
      discriminants.clear();
      result_type
    };

    let scope_raw = scope.as_ref() as *const _ as *mut _;

    for (def, partition) in refinements.iter() {
      let def_ty = self.lookup(scope, location, *def, false);
      let Some(def_ty) = def_ty else { continue };

      let mut ty = def_ty;

      let mut discriminants: Vec<TypeId> = Vec::new();
      let mut kind = RefinementsOpKind::None;

      let mut must_defer = must_defer_intersection(ty);

      for dt in &partition.discriminant_types {
        let dt_val = *dt;

        must_defer = must_defer || must_defer_intersection(dt_val);

        if must_defer {
          if kind == RefinementsOpKind::Intersect {
            ty = flush_constraints(self, kind, ty, &mut discriminants);
          }
          kind = RefinementsOpKind::Refine;
          discriminants.push(dt_val);
        } else {
          let status: ErrorSuppression = unsafe { should_suppress_errors(self.normalizer, ty) };

          if status.value == Value::NormalizationFailed {
            self.report_error(location, NormalizationTooComplex::default().into());
          }

          if kind == RefinementsOpKind::Refine {
            ty = flush_constraints(self, kind, ty, &mut discriminants);
          }
          kind = RefinementsOpKind::Intersect;

          discriminants.push(dt_val);

          if status.value == Value::Suppress {
            ty = flush_constraints(self, kind, ty, &mut discriminants);
            ty =
              self.make_union_scope_ptr_location_type_id_type_id(scope_raw, location, ty, unsafe {
                (*self.builtin_types).error_type
              });
          }
        }
      }

      if kind != RefinementsOpKind::None {
        ty = flush_constraints(self, kind, ty, &mut discriminants);
      }

      if partition.should_append_nil_type {
        ty = self.create_type_function_instance(
          unsafe { &(*self.builtin_types).type_functions.weakoptional_func },
          vec![ty],
          Vec::new(),
          scope,
          location,
        );
      }

      self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, *def, ty);
    }

    for c in constraints {
      self.add_constraint_scope_ptr_location_constraint_v(scope, location, c);
    }
  }
}
