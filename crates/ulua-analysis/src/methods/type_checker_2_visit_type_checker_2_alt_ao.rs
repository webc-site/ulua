//! `TypeChecker2::visit(AstExprUnary*)`（TypeChecker2.cpp 对照）。
use core::ptr::null;

use ulua_ast::{
  records::{
    ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
  },
  rtti::ast_node_is,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::{
    normalization_result::NormalizationResult, type_context::TypeContext,
    value_context::ValueContext,
  },
  functions::{
    find_metatable_entry::find_metatable_entry, first::first, follow_type::follow_type_id,
    get_type_alt_j::get_type_id, has_length::has_length, is_optional::is_optional,
  },
  records::{
    function_type::FunctionType, generic_error::GenericError,
    in_conditional_context::InConditionalContext,
    normalization_too_complex::NormalizationTooComplex, not_a_table::NotATable,
    optional_value_access::OptionalValueAccess, type_checker_2::TypeChecker2,
  },
  type_aliases::{
    type_error_data::{IntoTypeErrorData, TypeErrorData},
    type_id::TypeId,
  },
};
impl TypeChecker2 {
  pub fn visit_ast_expr_unary(&mut self, expr: &AstExprUnary) {
    let in_context = (expr.op != AstExprUnaryOp::Not).then(|| unsafe {
      InConditionalContext::new(
        &mut self.type_context as *mut TypeContext,
        TypeContext::Default,
      )
    });
    let _ = &in_context;

    self.visit_ast_expr_value_context(expr.expr, ValueContext::RValue);

    // SAFETY: expr.expr 指向 AST arena 节点，与 visit 树同寿。
    let operand_type = self.lookup_type(unsafe { &*expr.expr });
    let result_type = self.lookup_type(&expr.base);

    // SAFETY: 同上。
    if self
      .is_error_suppressing_location_type_id(unsafe { (*expr.expr).base.location }, operand_type)
    {
      return;
    }

    const K_UNARY_OP_METAMETHODS: [(AstExprUnaryOp, &str); 2] = [
      (AstExprUnaryOp::Minus, "__unm"),
      (AstExprUnaryOp::Len, "__len"),
    ];

    for (op, metamethod) in &K_UNARY_OP_METAMETHODS {
      if *op == expr.op {
        // SAFETY: self.module 与类型检查会话同寿（C++ 同契约）。
        let mm = unsafe {
          find_metatable_entry(
            self.builtin_types,
            &mut (*self.module).errors,
            operand_type,
            metamethod,
            expr.base.base.location,
          )
        };

        if let Some(mm_ty) = mm {
          if let Some(ftv) = get_type_id::<FunctionType>(follow_type_id(mm_ty)) {
            if let Some(ret) = first(ftv.ret_types, false) {
              if expr.op == AstExprUnaryOp::Len {
                // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
                self.test_is_subtype_type_id_type_id_location(
                  follow_type_id(ret),
                  unsafe { (*self.builtin_types).number_type },
                  expr.base.base.location,
                );
              }
            } else {
              self.report_error_type_error_data_location(
                TypeErrorData::GenericError(GenericError::new(alloc::format!(
                  "Metamethod '{}' must return a value",
                  metamethod
                ))),
                &expr.base.base.location,
              );
            }

            if first(ftv.arg_types, false).is_none() {
              self.report_error_type_error_data_location(
                TypeErrorData::GenericError(GenericError::new(
                  "__unm metamethod must accept one argument".to_string(),
                )),
                &expr.base.base.location,
              );
              return;
            }

            // SAFETY: self.module 同上；经裸指针 place 写 internal_types。
            let (expected_args, expected_ret) = unsafe {
              let arena = &mut (*self.module).internal_types;
              (
                arena.add_type_pack_initializer_list_type_id(&[operand_type]),
                arena.add_type_pack_initializer_list_type_id(&[result_type]),
              )
            };
            // SAFETY: 同上。
            let expected_function = unsafe {
              (*self.module)
                .internal_types
                .add_type(FunctionType::function_type_new(
                  expected_args,
                  expected_ret,
                  None,
                  false,
                ))
            };

            if !self.test_is_subtype_type_id_type_id_location(
              mm_ty,
              expected_function,
              expr.base.base.location,
            ) {
              return;
            }
          }
          return;
        }
        break;
      }
    }

    match expr.op {
      AstExprUnaryOp::Len => {
        let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null());
        let mut recursion_count = 0;
        let nty = self.normalizer.normalize(operand_type);

        if nty.should_suppress_errors() {
          return;
        }

        match self.normalizer.is_inhabited_normalized_type(&nty) {
          NormalizationResult::True => {}
          NormalizationResult::False => return,
          NormalizationResult::HitLimits => {
            self.report_error_type_error_data_location(
              NormalizationTooComplex::default().into_type_error_data(),
              &expr.base.base.location,
            );
            return;
          }
        }

        if !has_length(operand_type, &mut seen, &mut recursion_count) {
          if is_optional(operand_type) {
            self.report_error_type_error_data_location(
              OptionalValueAccess {
                optional: operand_type,
              }
              .into_type_error_data(),
              &expr.base.base.location,
            );
          } else {
            self.report_error_type_error_data_location(
              NotATable { ty: operand_type }.into_type_error_data(),
              &expr.base.base.location,
            );
          }
        }
      }
      AstExprUnaryOp::Minus => {
        // A negated integer literal is folded into one constant by the compiler, so it never negates anything.
        // SAFETY: expr.expr 指向 AST arena 节点。
        if ast_node_is::<AstExprConstantInteger>(unsafe { &*expr.expr }) {
          // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
          self.test_is_subtype_type_id_type_id_location(
            operand_type,
            unsafe { (*self.builtin_types).integer_type },
            expr.base.base.location,
          );
        } else {
          // SAFETY: 同上。
          self.test_is_subtype_type_id_type_id_location(
            operand_type,
            unsafe { (*self.builtin_types).number_type },
            expr.base.base.location,
          );
        }
      }
      AstExprUnaryOp::Not => {}
    }
  }
}
