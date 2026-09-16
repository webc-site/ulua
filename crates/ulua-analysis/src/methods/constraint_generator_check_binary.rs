//! Source: `Analysis/src/ConstraintGenerator.cpp:3606-3731` (hand-ported)
//! C++ `std::tuple<TypeId, TypeId, RefinementId> ConstraintGenerator::checkBinary(...)`.
use alloc::string::String;
use core::ptr::null_mut;

use ulua_ast::records::{ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, ast_node::AstNode};

use crate::{
  enums::type_context::TypeContext,
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id, has_tag_type_alt_b::has_tag,
    match_type_guard::match_type_guard,
  },
  records::{
    constraint_generator::ConstraintGenerator, extern_type::ExternType,
    in_conditional_context::InConditionalContext, union_type::UnionType,
  },
  type_aliases::{
    refinement_id_refinement::RefinementId, scope_ptr_type::ScopePtr, type_id::TypeId,
  },
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_binary(
    &mut self,
    scope: &ScopePtr,
    op: AstExprBinaryOp,
    left: *mut AstExpr,
    right: *mut AstExpr,
    expected_type: Option<TypeId>,
  ) -> (TypeId, TypeId, RefinementId) {
    unsafe {
      let _in_context = if op != AstExprBinaryOp::And
        && op != AstExprBinaryOp::Or
        && op != AstExprBinaryOp::CompareEq
        && op != AstExprBinaryOp::CompareNe
      {
        Some(InConditionalContext::new(
          &mut self.type_context,
          TypeContext::Default,
        ))
      } else {
        None
      };

      if op == AstExprBinaryOp::And {
        let mut relaxed_expected_lhs: Option<TypeId> = None;

        if let Some(exp) = expected_type {
          relaxed_expected_lhs = Some((*self.arena).add_type(UnionType {
            options: alloc::vec![(*self.builtin_types).falsy_type, exp],
          }));
        }

        let left_inf =
          self.check_scope_ptr_ast_expr_optional_type_id(scope, left, relaxed_expected_lhs);
        let left_type = left_inf.ty;
        let left_refinement = left_inf.refinement;

        let right_scope = self.child_scope(right as *mut AstNode, scope);
        self.apply_refinements(&right_scope, (*right).base.location, left_refinement);
        let right_inf =
          self.check_scope_ptr_ast_expr_optional_type_id(&right_scope, right, expected_type);
        let right_type = right_inf.ty;
        let right_refinement = right_inf.refinement;

        let conj = self
          .refinement_arena
          .conjunction_refinement_id_refinement_id(left_refinement, right_refinement);
        (left_type, right_type, conj)
      } else if op == AstExprBinaryOp::Or {
        let mut relaxed_expected_lhs: Option<TypeId> = None;

        if let Some(exp) = expected_type {
          relaxed_expected_lhs = Some((*self.arena).add_type(UnionType {
            options: alloc::vec![(*self.builtin_types).falsy_type, exp],
          }));
        }

        let left_inf =
          self.check_scope_ptr_ast_expr_optional_type_id(scope, left, relaxed_expected_lhs);
        let left_type = left_inf.ty;
        let left_refinement = left_inf.refinement;

        let right_scope = self.child_scope(right as *mut AstNode, scope);
        let negated = self
          .refinement_arena
          .negation_refinement_id(left_refinement);
        self.apply_refinements(&right_scope, (*right).base.location, negated);
        let right_inf =
          self.check_scope_ptr_ast_expr_optional_type_id(&right_scope, right, expected_type);
        let right_type = right_inf.ty;
        let right_refinement = right_inf.refinement;

        let disj = self
          .refinement_arena
          .disjunction_refinement_id_refinement_id(left_refinement, right_refinement);
        (left_type, right_type, disj)
      } else if let Some(typeguard) = match_type_guard(op as i32, left, right) {
        let left_type = self.check_scope_ptr_ast_expr(scope, left).ty;
        let right_type = self.check_scope_ptr_ast_expr(scope, right).ty;

        let key = (*self.dfg).get_refinement_key(typeguard.target as *const AstExpr);
        if key.is_null() {
          return (left_type, right_type, null_mut());
        }

        let mut discriminant_ty: TypeId = (*self.builtin_types).never_type;
        let guard_type = typeguard.r#type();
        if guard_type == "nil" {
          discriminant_ty = (*self.builtin_types).nil_type;
        } else if guard_type == "string" {
          discriminant_ty = (*self.builtin_types).string_type;
        } else if guard_type == "number" {
          discriminant_ty = (*self.builtin_types).number_type;
        } else if guard_type == "integer" {
          discriminant_ty = (*self.builtin_types).integer_type;
        } else if guard_type == "boolean" {
          discriminant_ty = (*self.builtin_types).boolean_type;
        } else if guard_type == "thread" {
          discriminant_ty = (*self.builtin_types).thread_type;
        } else if guard_type == "buffer" {
          discriminant_ty = (*self.builtin_types).buffer_type;
        } else if guard_type == "table" {
          discriminant_ty = (*self.builtin_types).table_type;
        } else if guard_type == "function" {
          discriminant_ty = (*self.builtin_types).function_type;
        } else if guard_type == "userdata" {
          // For now, we don't really care about being accurate with userdata if the typeguard was using typeof.
          discriminant_ty = (*self.builtin_types).extern_type;
        } else if guard_type == "vector" && !typeguard.is_typeof() {
          // `vector` is defined in EmbeddedBuiltinDefinitions, not as an actual built-in type
          let type_fun = self
            .global_scope
            .as_ref()
            .unwrap()
            .lookup_type(&String::from("vector"));
          if let Some(type_fun) = type_fun {
            discriminant_ty = follow_type_id(type_fun.r#type());
          }
        } else if !typeguard.is_typeof() {
          discriminant_ty = (*self.builtin_types).never_type;
        } else {
          let type_fun = self
            .global_scope
            .as_ref()
            .unwrap()
            .lookup_type(&String::from(guard_type));
          if let Some(type_fun) = type_fun
            && type_fun.type_params().is_empty()
            && type_fun.type_pack_params().is_empty()
          {
            let ty = follow_type_id(type_fun.r#type());

            // We're only interested in the root type of any extern type.
            // 对照 ConstraintGenerator.cpp:3729：`if (auto etv = get<ExternType>(ty); etv && ...)`
            if let Some(etv) = get_type_id::<ExternType>(ty)
              && (etv.parent == Some((*self.builtin_types).extern_type)
                || has_tag(ty, "typeofRoot"))
            {
              discriminant_ty = ty;
            }
          }
        }

        let proposition = self
          .refinement_arena
          .proposition_refinement_key_type_id(key, discriminant_ty);
        if op == AstExprBinaryOp::CompareEq {
          (left_type, right_type, proposition)
        } else if op == AstExprBinaryOp::CompareNe {
          let negated = self.refinement_arena.negation_refinement_id(proposition);
          (left_type, right_type, negated)
        } else {
          (*self.ice).ice_string("matchTypeGuard should only return a Some under `==` or `~=`!");
          (left_type, right_type, null_mut())
        }
      } else if op == AstExprBinaryOp::CompareEq || op == AstExprBinaryOp::CompareNe {
        // We are checking a binary expression of the form a op b
        // Just because a op b is expected to return a bool, doesn't mean a, b are expected to be bools too
        let left_type = self
          .check_scope_ptr_ast_expr_optional_type_id_bool(scope, left, None, true)
          .ty;
        let right_type = self
          .check_scope_ptr_ast_expr_optional_type_id_bool(scope, right, None, true)
          .ty;

        let left_key = (*self.dfg).get_refinement_key(left as *const AstExpr);
        let right_key = (*self.dfg).get_refinement_key(right as *const AstExpr);
        let mut left_refinement = self
          .refinement_arena
          .proposition_refinement_key_type_id(left_key, right_type);
        let mut right_refinement = self
          .refinement_arena
          .proposition_refinement_key_type_id(right_key, left_type);

        if op == AstExprBinaryOp::CompareNe {
          left_refinement = self
            .refinement_arena
            .negation_refinement_id(left_refinement);
          right_refinement = self
            .refinement_arena
            .negation_refinement_id(right_refinement);
        }

        let equiv = self
          .refinement_arena
          .equivalence_refinement_id_refinement_id(left_refinement, right_refinement);
        (left_type, right_type, equiv)
      } else {
        let left_type = self.check_scope_ptr_ast_expr(scope, left).ty;
        let right_type = self.check_scope_ptr_ast_expr(scope, right).ty;
        (left_type, right_type, null_mut())
      }
    }
  }
}
