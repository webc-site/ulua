//! Source: `Analysis/src/ConstraintGenerator.cpp:3606-3731` (hand-ported)
//! C++ `std::tuple<TypeId, TypeId, RefinementId> ConstraintGenerator::checkBinary(...)`.
use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp},
  rtti::AstNodePtr,
};

use crate::{
  enums::type_context::TypeContext,
  functions::{
    follow_type, get_type, has_tag_type::has_tag_type_id, match_type_guard::match_type_guard,
  },
  records::{
    arena_handle::alias_ref, constraint_generator::ConstraintGenerator, extern_type::ExternType,
    in_conditional_context::InConditionalContext, union_type::UnionType,
  },
  type_aliases::{
    refinement_id_refinement::{NULL_REFINEMENT_ID, RefinementId},
    scope_ptr_type::ScopePtr,
    type_id::TypeId,
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
  ) -> (TypeId, TypeId, Option<RefinementId>) {
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
          relaxed_expected_lhs = Some(self.arena.get_mut().add_type(UnionType {
            options: alloc::vec![self.builtin_types.get().falsy_type, exp],
          }));
        }

        let left_inf = self.check_expr_expected(scope, &*left, relaxed_expected_lhs);
        let left_type = left_inf.ty;
        let left_refinement = left_inf.refinement;

        let right_scope = self.child_scope(alias_ref(right.as_ast_node()), scope);
        self.apply_refinements(&right_scope, (*right).base.location, left_refinement);
        let right_inf = self.check_expr_expected(&right_scope, &*right, expected_type);
        let right_type = right_inf.ty;
        let right_refinement = right_inf.refinement;

        let conj = self
          .refinement_arena
          .conjunction_refinement_id_refinement_id(left_refinement, right_refinement);
        (left_type, right_type, conj)
      } else if op == AstExprBinaryOp::Or {
        let mut relaxed_expected_lhs: Option<TypeId> = None;

        if let Some(exp) = expected_type {
          relaxed_expected_lhs = Some(self.arena.get_mut().add_type(UnionType {
            options: alloc::vec![self.builtin_types.get().falsy_type, exp],
          }));
        }

        let left_inf = self.check_expr_expected(scope, &*left, relaxed_expected_lhs);
        let left_type = left_inf.ty;
        let left_refinement = left_inf.refinement;

        let right_scope = self.child_scope(alias_ref(right.as_ast_node()), scope);
        let negated = self
          .refinement_arena
          .negation_refinement_id(left_refinement);
        // §2：`None`（原 null 哨兵）与 apply_refinements 入口的判空 no-op 同义。
        if let Some(negated) = negated {
          self.apply_refinements(&right_scope, (*right).base.location, negated);
        }
        let right_inf = self.check_expr_expected(&right_scope, &*right, expected_type);
        let right_type = right_inf.ty;
        let right_refinement = right_inf.refinement;

        let disj = self
          .refinement_arena
          .disjunction_refinement_id_refinement_id(left_refinement, right_refinement);
        (left_type, right_type, disj)
      } else if let Some(typeguard) = match_type_guard(op as i32, left, right) {
        let left_type = self.check_expr(scope, &*left).ty;
        let right_type = self.check_expr(scope, &*right).ty;

        let key = (*self.dfg).get_refinement_key(typeguard.target as *const AstExpr);
        if key.is_null() {
          return (left_type, right_type, None);
        }

        let mut discriminant_ty: TypeId = self.builtin_types.get().never_type;
        let guard_type = typeguard.r#type();
        // kind 关键字单点派发：一次 match（字符串 DFA）替代 11 路 else-if 顺序串比较，
        // 形状对齐 type_checker_resolve_type_infer.rs 同场景收口；逐臂真值核对见提交说明。
        match guard_type {
          "nil" => discriminant_ty = self.builtin_types.get().nil_type,
          "string" => discriminant_ty = self.builtin_types.get().string_type,
          "number" => discriminant_ty = self.builtin_types.get().number_type,
          "integer" => discriminant_ty = self.builtin_types.get().integer_type,
          "boolean" => discriminant_ty = self.builtin_types.get().boolean_type,
          "thread" => discriminant_ty = self.builtin_types.get().thread_type,
          "buffer" => discriminant_ty = self.builtin_types.get().buffer_type,
          "table" => discriminant_ty = self.builtin_types.get().table_type,
          "function" => discriminant_ty = self.builtin_types.get().function_type,
          // For now, we don't really care about being accurate with userdata if the typeguard was using typeof.
          "userdata" => discriminant_ty = self.builtin_types.get().extern_type,
          // `vector` is defined in EmbeddedBuiltinDefinitions, not as an actual built-in type
          "vector" if !typeguard.is_typeof() => {
            // Safety: global_scope 由构造期注入非空空全局作用域（cpp NotNull<Scope>）。
            let type_fun = self
              .global_scope
              .as_ref()
              .expect("global_scope 构造期接线恒 Some（cpp NotNull<Scope>）")
              .lookup_type("vector");
            if let Some(type_fun) = type_fun {
              discriminant_ty = follow_type::follow(type_fun.r#type());
            }
          }
          other if typeguard.is_typeof() => {
            // Safety: 同上——global_scope 构造期接线不变式。
            let type_fun = self
              .global_scope
              .as_ref()
              .expect("global_scope 构造期接线恒 Some（cpp NotNull<Scope>）")
              .lookup_type(other);
            if let Some(type_fun) = type_fun
              && type_fun.type_params().is_empty()
              && type_fun.type_pack_params().is_empty()
            {
              let ty = follow_type::follow(type_fun.r#type());

              // We're only interested in the root type of any extern type.
              // 对照 ConstraintGenerator.cpp:3729：`if (auto etv = get<ExternType>(ty); etv && ...)`
              if let Some(etv) = get_type::get::<ExternType>(ty)
                && (etv.parent == Some(self.builtin_types.get().extern_type)
                  || has_tag_type_id(ty, "typeofRoot"))
              {
                discriminant_ty = ty;
              }
            }
          }
          _ => {} // 非 typeof 未知 kind：保持初值 never_type，等价原显式兜底臂。
        }

        let proposition = self
          .refinement_arena
          .proposition_refinement_key_type_id(key, discriminant_ty);
        if op == AstExprBinaryOp::CompareEq {
          (left_type, right_type, proposition)
        } else if op == AstExprBinaryOp::CompareNe {
          // §2：`None` 命题（原 null）经取反仍为 `None`（`negation(null) == null`），
          // 与 and_then 短路逐位同构。
          let negated = proposition.and_then(|p| self.refinement_arena.negation_refinement_id(p));
          (left_type, right_type, negated)
        } else {
          self
            .ice
            .get()
            .ice_string("matchTypeGuard should only return a Some under `==` or `~=`!");
          (left_type, right_type, None)
        }
      } else if matches!(op, AstExprBinaryOp::CompareEq | AstExprBinaryOp::CompareNe) {
        // We are checking a binary expression of the form a op b
        // Just because a op b is expected to return a bool, doesn't mean a, b are expected to be bools too
        let left_type = self.check_expr_singleton(scope, &*left, None, true).ty;
        let right_type = self.check_expr_singleton(scope, &*right, None, true).ty;

        let left_key = (*self.dfg).get_refinement_key(left as *const AstExpr);
        let right_key = (*self.dfg).get_refinement_key(right as *const AstExpr);
        let mut left_refinement = self
          .refinement_arena
          .proposition_refinement_key_type_id(left_key, right_type);
        let mut right_refinement = self
          .refinement_arena
          .proposition_refinement_key_type_id(right_key, left_type);

        if op == AstExprBinaryOp::CompareNe {
          left_refinement =
            left_refinement.and_then(|l| self.refinement_arena.negation_refinement_id(l));
          right_refinement =
            right_refinement.and_then(|r| self.refinement_arena.negation_refinement_id(r));
        }

        // 等价结点字段直存可空句柄：`None` 以定义处收口的具名哨兵落槽（原 cpp null）。
        let equiv = self
          .refinement_arena
          .equivalence_refinement_id_refinement_id(
            left_refinement.unwrap_or(NULL_REFINEMENT_ID),
            right_refinement.unwrap_or(NULL_REFINEMENT_ID),
          );
        (left_type, right_type, equiv)
      } else {
        let left_type = self.check_expr(scope, &*left).ty;
        let right_type = self.check_expr(scope, &*right).ty;
        (left_type, right_type, None)
      }
    }
  }
}
