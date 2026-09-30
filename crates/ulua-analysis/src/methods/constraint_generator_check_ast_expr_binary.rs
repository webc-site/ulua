use alloc::vec::Vec;

use ulua_ast::records::{ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, location::Location};
use ulua_common::macros::luau_assert::LUAU_UNREACHABLE;

use crate::{
  records::{
    constraint_generator::ConstraintGenerator, equality_constraint::EqualityConstraint,
    inference::Inference,
  },
  type_aliases::{
    constraint_v::ConstraintV, refinement_id_refinement::NULL_REFINEMENT_ID,
    scope_ptr_type::ScopePtr, type_id::TypeId,
  },
};

impl ConstraintGenerator {
  /// 对应 cpp `ConstraintGenerator::checkAstExprBinary`
  /// （`Analysis/src/ConstraintGenerator.cpp:3537-3600`）。`left`/`right` 为
  /// 刚经 RTTI 分发层确认的存活 `AstExpr` 子节点共享借用（cpp `AstExpr*` 的
  /// 引用化形态，调用点经 alias_ref 收口），直传 safe 的 `check_binary`。
  pub(crate) fn check_ast_expr_binary(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    op: AstExprBinaryOp,
    left: &AstExpr,
    right: &AstExpr,
    expected_type: Option<TypeId>,
  ) -> Inference {
    let (left_type, right_type, refinement) =
      self.check_binary(scope, op, left, right, expected_type);
    // §2：`check_binary` 以 `Option` 表达「无 refinement」（原 null 哨兵）；
    // `Inference.refinement` 是直存可空句柄的数据槽，在此以定义处收口的具名
    // 哨兵落槽，值面与原 cpp 逐位同构。
    let refinement = refinement.unwrap_or(NULL_REFINEMENT_ID);

    match op {
      AstExprBinaryOp::Add => {
        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.add_func,
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Sub => {
        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.sub_func,
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Mul => {
        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.mul_func,
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Div => {
        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.div_func,
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::FloorDiv => {
        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.idiv_func,
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Pow => {
        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.pow_func,
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Mod => {
        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.mod_func,
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Concat => {
        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.concat_func,
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::And => {
        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.and_func,
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Or => {
        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.or_func,
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
          self.builtin_types.get().boolean_type,
          refinement,
        )
      }
      AstExprBinaryOp::CompareEq | AstExprBinaryOp::CompareNe => {
        Inference::inference_type_id_refinement_id(
          self.builtin_types.get().boolean_type,
          refinement,
        )
      }
      AstExprBinaryOp::OpCount => {
        // Safety: `ice` 是构造期注入、等价 C++ `NotNull<InternalErrorReporter>`
        // 的存活报告器指针；OpCount 不是合法 AST 运算符，此分支按 cpp 原样
        // 上报 ICE 后即落入 LUAU_UNREACHABLE!（对应 `ice->ice(...)`）。
        self
          .ice
          .get()
          .ice_string("OpCount should never be generated in an AST.");
        LUAU_UNREACHABLE!()
      }
    }
  }
}
