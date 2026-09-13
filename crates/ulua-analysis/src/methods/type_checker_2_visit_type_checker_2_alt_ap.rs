//! `TypeChecker2::visit(AstExprBinary*, AstNode*)`（TypeChecker2.cpp 对照）。
use alloc::vec;

use ulua_ast::{
  functions::to_string_ast_alt_b::to_string,
  records::{
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_node::AstNode,
    ast_stat_compound_assign::AstStatCompoundAssign,
  },
};
use ulua_common::FFlag;

use crate::{
  enums::{
    normalization_result::NormalizationResult, op_kind::OpKind, type_context::TypeContext,
    value_context::ValueContext,
  },
  functions::{
    find_metatable_entry::find_metatable_entry, follow_type::follow_type_id,
    get_identifier_of_base_var_type_infer::get_identifier_of_base_var,
    get_metatable_type::get_metatable_type_id_not_null_builtin_types, get_type_alt_j::get_type_id,
    is_comparison_op::is_comparison_op, is_ok_to_compare::is_ok_to_compare, is_string::is_string,
    op_to_meta_table_entry::op_to_meta_table_entry, strip_nil::strip_nil,
    to_string_to_string_alt_c::to_string_type_id,
  },
  records::{
    any_type::AnyType, blocked_type::BlockedType,
    cannot_compare_unrelated_types::CannotCompareUnrelatedTypes,
    cannot_infer_binary_operation::CannotInferBinaryOperation, free_type::FreeType,
    generic_error::GenericError, generic_type::GenericType,
    in_conditional_context::InConditionalContext, never_type::NeverType,
    normalization_too_complex::NormalizationTooComplex, table_type::TableType,
    type_checker_2::TypeChecker2, type_function_instance_type::TypeFunctionInstanceType,
    union_type::UnionType,
  },
  type_aliases::{
    error_type::ErrorType,
    type_error_data::{IntoTypeErrorData, TypeErrorData},
  },
};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr_binary_ast_node(
    &mut self,
    expr: &AstExprBinary,
    override_key: *mut AstNode,
  ) {
    let op = expr.op;
    let in_context = (!matches!(
      op,
      AstExprBinaryOp::And
        | AstExprBinaryOp::Or
        | AstExprBinaryOp::CompareEq
        | AstExprBinaryOp::CompareNe
    ))
    .then(|| unsafe {
      InConditionalContext::new(
        &mut self.type_context as *mut TypeContext,
        TypeContext::Default,
      )
    });

    if FFlag::LuauLValueCompoundAssignmentVisitLhs.get() {
      // In compound assignments, the left side is both read-from and written-to, so we have to visit it in both contexts.
      // SAFETY: override_key 由调用方（compound assign 分支）保证有效或为 null。
      if !override_key.is_null() && unsafe { (*override_key).is::<AstStatCompoundAssign>() } {
        self.visit_ast_expr_value_context(expr.left, ValueContext::LValue);
      }
    }

    self.visit_ast_expr_value_context(expr.left, ValueContext::RValue);
    self.visit_ast_expr_value_context(expr.right, ValueContext::RValue);

    let scope = *self
      .stack
      .last()
      .expect("TypeChecker2 stack should not be empty");

    let is_equality =
      expr.op == AstExprBinaryOp::CompareEq || expr.op == AstExprBinaryOp::CompareNe;
    let is_comparison = is_comparison_op(expr.op);
    let is_logical = expr.op == AstExprBinaryOp::And || expr.op == AstExprBinaryOp::Or;

    // SAFETY: left/right 均指向 AST arena 节点；expr 自身经 &expr.base 视作 AstExpr。
    let mut left_type = follow_type_id(self.lookup_type(unsafe { &*expr.left }));
    let right_type = follow_type_id(self.lookup_type(unsafe { &*expr.right }));
    let expected_result = follow_type_id(self.lookup_type(&expr.base));
    if get_type_id::<TypeFunctionInstanceType>(expected_result).is_some() {
      self.check_for_internal_type_function(expected_result, expr.base.base.location);
      return;
    }

    if expr.op == AstExprBinaryOp::Or {
      // SAFETY: self.module 与类型检查会话同寿（C++ 同契约）。
      left_type = unsafe {
        strip_nil(
          self.builtin_types,
          &mut (*self.module).internal_types,
          left_type,
        )
      };
    }

    let norm_left = self.normalizer.try_normalize(left_type);
    let norm_right = self.normalizer.try_normalize(right_type);

    let is_string_operation = norm_left
      .as_ref()
      .map_or_else(|| is_string(left_type), |norm| norm.is_subtype_of_string())
      && norm_right
        .as_ref()
        .map_or_else(|| is_string(right_type), |norm| norm.is_subtype_of_string());

    left_type = follow_type_id(left_type);
    if get_type_id::<AnyType>(left_type).is_some()
      || get_type_id::<ErrorType>(left_type).is_some()
      || get_type_id::<NeverType>(left_type).is_some()
      || get_type_id::<AnyType>(right_type).is_some()
      || get_type_id::<ErrorType>(right_type).is_some()
      || get_type_id::<NeverType>(right_type).is_some()
      || norm_left
        .as_ref()
        .is_some_and(|norm| norm.should_suppress_errors())
      || norm_right
        .as_ref()
        .is_some_and(|norm| norm.should_suppress_errors())
    {
      return;
    }

    if (get_type_id::<BlockedType>(left_type).is_some()
      || get_type_id::<FreeType>(left_type).is_some()
      || get_type_id::<GenericType>(left_type).is_some())
      && !is_equality
      && !is_logical
    {
      // get_identifier_of_base_var（清单外）收裸指针；left 指向 AST arena。
      let name = get_identifier_of_base_var(expr.left);

      self.report_error_type_error_data_location(
        TypeErrorData::CannotInferBinaryOperation(CannotInferBinaryOperation::new(
          expr.op,
          name,
          if is_comparison {
            OpKind::Comparison
          } else {
            OpKind::Operation
          },
        )),
        &expr.base.base.location,
      );
      return;
    }

    let types_have_intersection = self
      .normalizer
      .is_intersection_inhabited_type_id_type_id(left_type, right_type);

    if types_have_intersection == NormalizationResult::HitLimits {
      self.report_error_type_error_data_location(
        NormalizationTooComplex::default().into_type_error_data(),
        &expr.base.base.location,
      );
      return;
    }

    if is_equality || is_comparison {
      if !is_ok_to_compare(
        &mut self.normalizer,
        types_have_intersection,
        norm_left.as_deref(),
        norm_right.as_deref(),
      ) {
        self.report_error_type_error_data_location(
          TypeErrorData::CannotCompareUnrelatedTypes(CannotCompareUnrelatedTypes {
            left: left_type,
            right: right_type,
            op: expr.op,
          }),
          &expr.base.base.location,
        );
        return;
      }

      let either_expr_is_nil = norm_left.as_ref().is_some_and(|norm| norm.is_nil())
        || norm_right.as_ref().is_some_and(|norm| norm.is_nil());

      if is_equality && either_expr_is_nil {
        return;
      }
    }

    if is_logical || (is_comparison && is_string_operation) {
      return;
    }

    let metamethod = op_to_meta_table_entry(expr.op);
    if !metamethod.is_empty() {
      // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
      let (left_mt, right_mt) = unsafe {
        (
          get_metatable_type_id_not_null_builtin_types(left_type, &*self.builtin_types),
          get_metatable_type_id_not_null_builtin_types(right_type, &*self.builtin_types),
        )
      };
      let mut matches = left_mt == right_mt;

      if is_equality && !matches {
        if !matches
          && right_mt.is_some()
          && let Some(utv) = get_type_id::<UnionType>(left_type)
        {
          for &option in &utv.options {
            // SAFETY: 同上。
            if unsafe {
              get_metatable_type_id_not_null_builtin_types(
                follow_type_id(option),
                &*self.builtin_types,
              ) == right_mt
            } {
              matches = true;
              break;
            }
          }
        }

        if !matches
          && left_mt.is_some()
          && let Some(utv) = get_type_id::<UnionType>(right_type)
        {
          for &option in &utv.options {
            // SAFETY: 同上。
            if unsafe {
              get_metatable_type_id_not_null_builtin_types(
                follow_type_id(option),
                &*self.builtin_types,
              ) == left_mt
            } {
              matches = true;
              break;
            }
          }
        }
      }

      if get_type_id::<TableType>(left_type).is_none()
        && get_type_id::<TableType>(right_type).is_none()
        && (left_mt.is_none() || right_mt.is_none())
      {
        matches = matches || types_have_intersection != NormalizationResult::False;
      }

      if !matches && is_comparison {
        self.report_error_type_error_data_location(
          TypeErrorData::GenericError(GenericError::new(alloc::format!(
            "Types {} and {} cannot be compared with {} because they do not have the same metatable",
            to_string_type_id(left_type),
            to_string_type_id(right_type),
            to_string(expr.op)
          ))),
          &expr.base.base.location,
        );
        return;
      }

      // SAFETY: self.module 同上。
      let left_mm = unsafe {
        find_metatable_entry(
          self.builtin_types,
          &mut (*self.module).errors,
          left_type,
          &metamethod,
          expr.base.base.location,
        )
      };
      let right_mm = if left_mm.is_none() {
        unsafe {
          find_metatable_entry(
            self.builtin_types,
            // SAFETY: 同上。
            &mut (*self.module).errors,
            right_type,
            &metamethod,
            expr.base.base.location,
          )
        }
      } else {
        None
      };

      if left_mm.or(right_mm).is_some() {
        return;
      }

      if !is_equality
        && !(is_string_operation && (expr.op == AstExprBinaryOp::Concat || is_comparison))
      {
        if (left_mt.is_some() && !is_string(left_type))
          || (right_mt.is_some() && !is_string(right_type))
        {
          if is_comparison {
            self.report_error_type_error_data_location(
              TypeErrorData::CannotCompareUnrelatedTypes(CannotCompareUnrelatedTypes {
                left: left_type,
                right: right_type,
                op: expr.op,
              }),
              &expr.base.base.location,
            );
          } else {
            self.report_error_type_error_data_location(
              TypeErrorData::GenericError(GenericError::new(alloc::format!(
                "Operator {} is not applicable for '{}' and '{}' because neither type's metatable has a '{}' metamethod",
                to_string(expr.op),
                to_string_type_id(left_type),
                to_string_type_id(right_type),
                metamethod
              ))),
              &expr.base.base.location,
            );
          }
          return;
        } else if left_mt.is_none()
          && right_mt.is_none()
          && (get_type_id::<TableType>(left_type).is_some()
            || get_type_id::<TableType>(right_type).is_some())
        {
          if is_comparison {
            self.report_error_type_error_data_location(
              TypeErrorData::CannotCompareUnrelatedTypes(CannotCompareUnrelatedTypes {
                left: left_type,
                right: right_type,
                op: expr.op,
              }),
              &expr.base.base.location,
            );
          } else {
            self.report_error_type_error_data_location(
              TypeErrorData::GenericError(GenericError::new(alloc::format!(
                "Operator {} is not applicable for '{}' and '{}' because neither type has a metatable",
                to_string(expr.op),
                to_string_type_id(left_type),
                to_string_type_id(right_type)
              ))),
              &expr.base.base.location,
            );
          }
          return;
        }
      }
    }

    match expr.op {
      AstExprBinaryOp::Add
      | AstExprBinaryOp::Sub
      | AstExprBinaryOp::Mul
      | AstExprBinaryOp::Div
      | AstExprBinaryOp::FloorDiv
      | AstExprBinaryOp::Pow
      | AstExprBinaryOp::Mod => {
        // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
        let number_type = unsafe { (*self.builtin_types).number_type };
        self.test_is_subtype_type_id_type_id_location(
          left_type,
          number_type,
          // SAFETY: left 指向 AST arena 节点。
          unsafe { (*expr.left).base.location },
        );
        self.test_is_subtype_type_id_type_id_location(
          right_type,
          number_type,
          // SAFETY: right 指向 AST arena 节点。
          unsafe { (*expr.right).base.location },
        );
      }
      AstExprBinaryOp::Concat => {
        // SAFETY: self.module/builtin_types 同上；经裸指针 place 写 internal_types。
        let number_or_string = unsafe {
          let bts = &*self.builtin_types;
          (*self.module).internal_types.add_type(UnionType {
            options: vec![bts.number_type, bts.string_type],
          })
        };
        self.test_is_subtype_type_id_type_id_location(
          left_type,
          number_or_string,
          // SAFETY: left 指向 AST arena 节点。
          unsafe { (*expr.left).base.location },
        );
        self.test_is_subtype_type_id_type_id_location(
          right_type,
          number_or_string,
          // SAFETY: right 指向 AST arena 节点。
          unsafe { (*expr.right).base.location },
        );
      }
      AstExprBinaryOp::CompareGe
      | AstExprBinaryOp::CompareGt
      | AstExprBinaryOp::CompareLe
      | AstExprBinaryOp::CompareLt => {
        if norm_left
          .as_ref()
          .is_some_and(|norm| norm.should_suppress_errors())
        {
          return;
        }

        if norm_left.as_ref().is_some_and(|norm| {
          self.normalizer.is_inhabited_normalized_type(norm) == NormalizationResult::False
        }) {
          return;
        }

        // SAFETY: self.subtyping/builtin_types 由构造方保证有效（C++ 同契约）。
        if unsafe {
          (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
            left_type,
            (*self.builtin_types).number_type,
            scope,
          )
        }
        .is_subtype
        {
          self.test_is_subtype_type_id_type_id_location(
            right_type,
            // SAFETY: 同上。
            unsafe { (*self.builtin_types).number_type },
            // SAFETY: right 指向 AST arena 节点。
            unsafe { (*expr.right).base.location },
          );
          return;
        }

        // SAFETY: 同上。
        if unsafe {
          (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
            left_type,
            (*self.builtin_types).string_type,
            scope,
          )
        }
        .is_subtype
        {
          self.test_is_subtype_type_id_type_id_location(
            right_type,
            // SAFETY: 同上。
            unsafe { (*self.builtin_types).string_type },
            // SAFETY: right 指向 AST arena 节点。
            unsafe { (*expr.right).base.location },
          );
          return;
        }

        self.report_error_type_error_data_location(
          TypeErrorData::GenericError(GenericError::new(alloc::format!(
            "Types '{}' and '{}' cannot be compared with relational operator {}",
            to_string_type_id(left_type),
            to_string_type_id(right_type),
            to_string(expr.op)
          ))),
          &expr.base.base.location,
        );
        return;
      }
      AstExprBinaryOp::And
      | AstExprBinaryOp::Or
      | AstExprBinaryOp::CompareEq
      | AstExprBinaryOp::CompareNe
      | AstExprBinaryOp::OpCount => {}
    }

    let _ = in_context;
    let _ = types_have_intersection;
  }
}
