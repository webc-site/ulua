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
  pub(crate) fn apply_refinements(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    refinement: RefinementId,
  ) {
    // DELIBERATE DEVIATION：`RefinementId` 是 refinement arena 的身份裸句柄，
    // 数据槽（`Inference.refinement`、`Vec<RefinementId>`）以定义处收口的具名哨兵
    // `NULL_REFINEMENT_ID` 表达 cpp `nullptr`（review.md §10 门面收口），故本入口
    // 仍按哨兵判空早退，而非把整条 refinement 链改成 `Option`。
    if refinement.is_null() {
      return;
    }

    let mut refinements: RefinementContext = RefinementContext::default();
    let mut constraints: Vec<ConstraintV> = Vec::new();

    // compute_refinement 已引用签名化：scope 共享借用直传，输出上下文/队列为
    // 本帧局部的独占借用。
    self.compute_refinement(
      scope,
      location,
      refinement,
      &mut refinements,
      true,
      false,
      &mut constraints,
    );

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

      let mut args: Vec<TypeId> = Vec::with_capacity(1 + discriminants.len());
      args.push(ty);

      // `this.builtin_types` 为构造期接线的非空 Handle（比 this 长寿）；只读借用
      // 指向其 type_functions 持久字段。
      let type_functions = &this.builtin_types.get().type_functions;

      let func = if kind == RefinementsOpKind::Intersect {
        &type_functions.intersect_func
      } else {
        &type_functions.refine_func
      };

      LUAU_ASSERT!(!func.name.is_empty());
      args.extend_from_slice(discriminants);

      let result_type = this.create_type_function_instance(func, args, Vec::new(), scope, location);
      discriminants.clear();
      result_type
    };

    for (def, partition) in refinements.iter() {
      let Some(mut ty) = self.lookup(scope, location, *def, false) else {
        continue;
      };

      let mut discriminants: Vec<TypeId> = Vec::new();
      let mut kind = RefinementsOpKind::None;

      let mut must_defer = must_defer_intersection(ty);

      for &dt_val in &partition.discriminant_types {
        must_defer = must_defer || must_defer_intersection(dt_val);

        if must_defer {
          if kind == RefinementsOpKind::Intersect {
            ty = flush_constraints(self, kind, ty, &mut discriminants);
          }
          kind = RefinementsOpKind::Refine;
          discriminants.push(dt_val);
        } else {
          // self.normalizer 为构造时接线、比 self 长寿的存活句柄，经
          // Handle::get_mut 以 &mut 直传 should_suppress_errors（只读归一化判定）。
          let status: ErrorSuppression = should_suppress_errors(self.normalizer.get_mut(), ty);

          if status.value == Value::NormalizationFailed {
            self.report_error(location, NormalizationTooComplex.into());
          }

          if kind == RefinementsOpKind::Refine {
            ty = flush_constraints(self, kind, ty, &mut discriminants);
          }
          kind = RefinementsOpKind::Intersect;

          discriminants.push(dt_val);

          if status.value == Value::Suppress {
            ty = flush_constraints(self, kind, ty, &mut discriminants);
            // `self.builtin_types` 为构造期接线的非空 Handle，只读 error_type 常量槽。
            let error_type = self.builtin_types.get().error_type;
            ty =
              self.make_union_scope_ptr_location_type_id_type_id(scope, location, ty, error_type);
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
