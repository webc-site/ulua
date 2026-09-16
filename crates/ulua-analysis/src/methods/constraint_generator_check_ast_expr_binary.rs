use alloc::vec::Vec;

use ulua_ast::records::{ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, location::Location};
use ulua_common::macros::luau_unreachable::LUAU_UNREACHABLE;

use crate::{
  records::{
    constraint_generator::ConstraintGenerator, equality_constraint::EqualityConstraint,
    inference::Inference,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `left、`right` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn check_ast_expr_binary(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    op: AstExprBinaryOp,
    left: *mut AstExpr,
    right: *mut AstExpr,
    expected_type: Option<TypeId>,
  ) -> Inference {
    let (left_type, right_type, refinement) =
      unsafe { self.check_binary(scope, op, left, right, expected_type) };

    match op {
      AstExprBinaryOp::Add => {
        let result_type = self.create_type_function_instance(
          unsafe { &(*self.builtin_types).type_functions.add_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Sub => {
        let result_type = self.create_type_function_instance(
          unsafe { &(*self.builtin_types).type_functions.sub_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Mul => {
        let result_type = self.create_type_function_instance(
          unsafe { &(*self.builtin_types).type_functions.mul_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Div => {
        let result_type = self.create_type_function_instance(
          unsafe { &(*self.builtin_types).type_functions.div_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::FloorDiv => {
        let result_type = self.create_type_function_instance(
          unsafe { &(*self.builtin_types).type_functions.idiv_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Pow => {
        let result_type = self.create_type_function_instance(
          unsafe { &(*self.builtin_types).type_functions.pow_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Mod => {
        let result_type = self.create_type_function_instance(
          unsafe { &(*self.builtin_types).type_functions.mod_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Concat => {
        let result_type = self.create_type_function_instance(
          unsafe { &(*self.builtin_types).type_functions.concat_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::And => {
        let result_type = self.create_type_function_instance(
          unsafe { &(*self.builtin_types).type_functions.and_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Or => {
        let result_type = self.create_type_function_instance(
          unsafe { &(*self.builtin_types).type_functions.or_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::CompareLt
      | AstExprBinaryOp::CompareGe
      | AstExprBinaryOp::CompareLe
      | AstExprBinaryOp::CompareGt => {
        self.add_constraint_scope_ptr_location_constraint_v(
          scope,
          location,
          ConstraintV::Equality(EqualityConstraint {
            result_type: left_type,
            assignment_type: right_type,
          }),
        );
        Inference::inference_type_id_refinement_id(
          unsafe { (*self.builtin_types).boolean_type },
          refinement,
        )
      }
      AstExprBinaryOp::CompareEq | AstExprBinaryOp::CompareNe => {
        Inference::inference_type_id_refinement_id(
          unsafe { (*self.builtin_types).boolean_type },
          refinement,
        )
      }
      AstExprBinaryOp::OpCount => {
        unsafe { (*self.ice).ice_string("OpCount should never be generated in an AST.") };
        LUAU_UNREACHABLE!()
      }
    }
  }
}
