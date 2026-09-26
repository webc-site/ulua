//! Source: `Analysis/src/ControlFlowGraph.cpp:368-422` (hand-ported)
//! C++ `std::optional<RefinementId> CFGBuilder::resolveCondition(AstExpr* condition)`.
use alloc::string::ToString;

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_group::AstExprGroup,
    ast_expr_local::AstExprLocal,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
  },
  rtti::{AstNodePtr, ast_node_try_as_ptr},
};

use crate::{
  functions::match_type_guard::match_type_guard,
  methods::refinement_arena_type_proposition::refinement_arena_type_proposition,
  records::{cfg_builder::CfgBuilder, symbol::Symbol},
  type_aliases::{
    def_id_control_flow_graph::DefId, refinement_id_control_flow_graph::RefinementId,
  },
};
impl CfgBuilder {
  pub fn resolve_condition(&mut self, condition: *mut AstExpr) -> Option<RefinementId> {
    unsafe {
      // auto& arena = allocator->refinementArena;  (accessed per-use below)
      // 判型+判空三步样板折叠为 ast_node_try_as_ptr Option 门面（cpp
      // `condition->as<T>()` 两段式）；use_defs 身份键沿用同一节点的基类
      // 指针视图（condition 与命中下转恒同址，repr(C) 首字段）。

      // if (auto group = condition->as<AstExprGroup>())
      //     return resolveCondition(group->expr);
      // expr 已句柄化恒非空；resolve_condition 为既有裸指针 API，经 as_ptr 桥接。
      if let Some(group) = ast_node_try_as_ptr::<AstExprGroup>(condition.as_ast_node()) {
        return self.resolve_condition(group.expr.as_ptr());
      }

      // else if (auto loc = condition->as<AstExprLocal>()) { ... }
      if let Some(loc) = ast_node_try_as_ptr::<AstExprLocal>(condition.as_ast_node()) {
        // DefId def = readVariable(currentBlock, Symbol(loc->local));
        let def: DefId =
          self.read_variable(self.current_block, Symbol::from_local(loc.local.as_ptr()));
        // cfg->useDefs[loc] = def;
        *self
          .cfg
          .as_mut()
          .expect("函数契约保证 cfg 为 Some，取回必命中")
          .use_defs
          .get_or_insert(condition.cast::<AstExpr>()) = def;
        // return arena.proposition(def, /* sense */ true);
        return Some(
          (*self.allocator)
            .refinement_arena
            .proposition_def_id_bool(def, true),
        );
      }

      // else if (auto binop = condition->as<AstExprBinary>()) { ... }
      if let Some(binop) = ast_node_try_as_ptr::<AstExprBinary>(condition.as_ast_node()) {
        let op = binop.op;
        // if (auto tg = matchTypeGuard(binop->op, binop->left, binop->right)) { ... }
        // left/right 已句柄化恒非空；match_type_guard 为既有裸指针 API，经 as_ptr 桥接。
        if let Some(tg) = match_type_guard(op as i32, binop.left.as_ptr(), binop.right.as_ptr()) {
          // if (auto tgtLocal = tg->target->as<AstExprLocal>()) { ... }
          if let Some(tgt_local) = ast_node_try_as_ptr::<AstExprLocal>(tg.target()) {
            // auto def = readVariable(currentBlock, Symbol(tgtLocal->local));
            let def = self.read_variable(
              self.current_block,
              Symbol::from_local(tgt_local.local.as_ptr()),
            );
            // cfg->useDefs[tgtLocal] = def;
            *self
              .cfg
              .as_mut()
              .expect("函数契约保证 cfg 为 Some，取回必命中")
              .use_defs
              .get_or_insert(tg.target().cast::<AstExpr>()) = def;
            // bool sense = binop->op == AstExprBinary::CompareEq;
            let sense = op == AstExprBinaryOp::CompareEq;
            // return arena.typeProposition(def, tg->type, tg->is_typeof, sense);
            let arena = &mut (*self.allocator).refinement_arena;
            return Some(refinement_arena_type_proposition(
              arena,
              def,
              Some(tg.r#type().to_string()),
              tg.is_typeof(),
              sense,
            ));
          }
          // return std::nullopt;
          return None;
        }

        // auto lRef = resolveCondition(binop->left);
        // auto rRef = resolveCondition(binop->right);
        // 同上：resolve_condition 既有裸指针 API 经 as_ptr 桥接。
        let l_ref = self.resolve_condition(binop.left.as_ptr());
        let r_ref = self.resolve_condition(binop.right.as_ptr());
        if op == AstExprBinaryOp::And {
          // (A and B) truthy => both truthy; a missing side still preserves the other.
          if let (Some(l), Some(r)) = (l_ref, r_ref) {
            return Some((*self.allocator).refinement_arena.conjunction_mut(l, r));
          }
          return if l_ref.is_some() { l_ref } else { r_ref };
        } else if op == AstExprBinaryOp::Or {
          // (A or B) truthy => at least one truthy; an unrefined side means we can't narrow.
          if let (Some(l), Some(r)) = (l_ref, r_ref) {
            return Some((*self.allocator).refinement_arena.disjunction_mut(l, r));
          }
        }
        // falls through to `return std::nullopt;`
        return None;
      }

      // else if (auto unop = condition->as<AstExprUnary>()) { ... }
      if let Some(unop) = ast_node_try_as_ptr::<AstExprUnary>(condition.as_ast_node()) {
        // if (unop->op == AstExprUnary::Not)
        if unop.op == AstExprUnaryOp::Not {
          // if (auto inner = resolveCondition(unop->expr))
          //     return arena.negation(*inner);
          // expr 已句柄化；resolve_condition 为既有裸指针 API，经 as_ptr 桥接。
          if let Some(inner) = self.resolve_condition(unop.expr.as_ptr()) {
            return Some((*self.allocator).refinement_arena.negation_mut(inner));
          }
        }
      }

      // return std::nullopt;
      None
    }
  }
}
