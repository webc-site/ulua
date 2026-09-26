use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{refinements_op_kind::RefinementsOpKind, value::Value},
  functions::{
    arc_as_mut::arc_as_mut, must_defer_intersection::must_defer_intersection,
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

    // Safety: arc_as_mut(scope) 由调用方持有的存活 `Arc<Scope>`（scope: &ScopePtr）得到
    // *mut Scope，同步调用期内该 Arc 存活；compute_refinement 依 C++ `const ScopePtr&`
    // 语义经此句柄访问 scope，单线程串行无并发别名。
    unsafe {
      self.compute_refinement(
        arc_as_mut(scope),
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
      // builtin_types 为 Handle（构造时接线非空、比 this 长寿）；只读借用指向
      // 其 type_functions 持久字段。
      let type_functions = &builtin_types.get().type_functions;

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

    let scope_raw = arc_as_mut(scope);

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
          // Safety: self.normalizer.as_ptr() 为构造时接线的非空 *mut Normalizer，比 self 长寿；
          // should_suppress_errors 依其契约解引用该 normalizer 做归一化判定（只读语义）。
          let status: ErrorSuppression =
            unsafe { should_suppress_errors(self.normalizer.as_ptr(), ty) };

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
            ty = self.make_union_scope_ptr_location_type_id_type_id(scope_raw, location, ty, {
              // Safety: self.builtin_types.as_ptr() 构造时接线非空存活，此处仅只读 error_type 字段。
              self.builtin_types.get().error_type
            });
          }
        }
      }

      if kind != RefinementsOpKind::None {
        ty = flush_constraints(self, kind, ty, &mut discriminants);
      }

      if partition.should_append_nil_type {
        ty = self.create_type_function_instance(
          {
            // Safety: self.builtin_types.as_ptr() 非空存活（构造接线），取只读借用指向其
            // type_functions.weakoptional_func 持久字段，无并发可变借用。
            &self.builtin_types.get().type_functions.weakoptional_func
          },
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
