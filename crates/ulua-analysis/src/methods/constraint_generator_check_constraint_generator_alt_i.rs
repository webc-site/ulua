//! Source: `Analysis/src/ConstraintGenerator.cpp:3384-3412` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprUnary* unary)`.
use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
  },
  rtti::ast_node_is,
};

use crate::{
  enums::type_context::TypeContext,
  records::{
    constraint_generator::ConstraintGenerator, in_conditional_context::InConditionalContext,
    inference::Inference,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_scope_ptr_ast_expr_unary(
    &mut self,
    scope: &ScopePtr,
    unary: *mut AstExprUnary,
  ) -> Inference {
    unsafe {
      let op = (*unary).op;

      let _in_context = if op != AstExprUnaryOp::Not {
        Some(InConditionalContext::new(
          &mut self.type_context,
          TypeContext::Default,
        ))
      } else {
        None
      };

      let inf = self.check_scope_ptr_ast_expr(scope, (*unary).expr);
      let operand_type = inf.ty;
      let refinement = inf.refinement;

      match op {
        AstExprUnaryOp::Not => {
          let not_func = &(*self.builtin_types).type_functions.not_func as *const _;
          let result_type = self.create_type_function_instance(
            &*not_func,
            alloc::vec![operand_type],
            Vec::new(),
            scope,
            (*unary).base.base.location,
          );
          let negated = self.refinement_arena.negation_refinement_id(refinement);
          Inference::inference_type_id_refinement_id(result_type, negated)
        }
        AstExprUnaryOp::Len => {
          let len_func = &(*self.builtin_types).type_functions.len_func as *const _;
          let result_type = self.create_type_function_instance(
            &*len_func,
            alloc::vec![operand_type],
            Vec::new(),
            scope,
            (*unary).base.base.location,
          );
          Inference::inference_type_id_refinement_id(result_type, refinement)
        }
        AstExprUnaryOp::Minus => {
          // compileExprUnary folds `-1i` into one negative constant, so a negated integer literal is a value rather than
          // an operation. A non-literal integer still reaches the runtime, which has no __unm, so it keeps the check.
          if ast_node_is::<AstExprConstantInteger>(&*(*unary).expr) {
            return Inference::inference_type_id_refinement_id(
              (*self.builtin_types).integer_type,
              refinement,
            );
          }

          let unm_func = &(*self.builtin_types).type_functions.unm_func as *const _;
          let result_type = self.create_type_function_instance(
            &*unm_func,
            alloc::vec![operand_type],
            Vec::new(),
            scope,
            (*unary).base.base.location,
          );
          Inference::inference_type_id_refinement_id(result_type, refinement)
        }
      }
    }
  }
}
