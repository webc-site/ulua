//! Source: `Analysis/src/ControlFlowGraph.cpp:368-422` (hand-ported)
//! C++ `std::optional<RefinementId> CFGBuilder::resolveCondition(AstExpr* condition)`.
use alloc::string::ToString;
use core::ptr::from_ref;

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_group::AstExprGroup,
    ast_expr_local::AstExprLocal,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    node_handle::OptNode,
  },
  rtti::ast_node_try_as,
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
    // `condition` 出自 CFG 构造期存活的 AST arena（records 引用化波次前的裸
    // 指针入参形态）：先经局部 `OptNode` 句柄把判空与借用折进本函数半径，
    // 判型全部走生命周期正确的 `ast_node_try_as`；use_defs 只存身份指针
    //（`from_ref(...).cast_mut()` 与入参同址，repr(C) 首字段，不解引用）。
    let cond = OptNode::from_ptr(condition);
    let expr = cond.get()?;

    {
      // if (auto group = condition->as<AstExprGroup>())
      //     return resolveCondition(group->expr);
      // expr 已句柄化恒非空；resolve_condition 为既有裸指针 API，经 as_ptr 桥接。
      if let Some(group) = ast_node_try_as::<AstExprGroup>(expr) {
        return self.resolve_condition(group.expr.as_ptr());
      }

      // else if (auto loc = condition->as<AstExprLocal>()) { ... }
      if let Some(loc) = ast_node_try_as::<AstExprLocal>(expr) {
        // DefId def = readVariable(currentBlock, Symbol(loc->local));
        let def: DefId =
          self.read_variable(self.current_block, Symbol::from_local(loc.local.as_ptr()));
        // cfg->useDefs[loc] = def;
        *self
          .cfg
          .as_mut()
          .expect("函数契约保证 cfg 为 Some，取回必命中")
          .use_defs
          .get_or_insert(from_ref(expr).cast_mut()) = def;
        // return arena.proposition(def, /* sense */ true);
        // arena 句柄经 `get_mut` 物化本帧独占借用（arena_handle 模块契约，
        // 对应 cpp `allocator->refinementArena`），原裸指针 deref 的 unsafe 消亡。
        return Some(
          self
            .allocator
            .get_mut()
            .refinement_arena
            .proposition_def_id_bool(def, true),
        );
      }

      // else if (auto binop = condition->as<AstExprBinary>()) { ... }
      if let Some(binop) = ast_node_try_as::<AstExprBinary>(expr) {
        let op = binop.op;
        // if (auto tg = matchTypeGuard(binop->op, binop->left, binop->right)) { ... }
        // left/right 已句柄化恒非空；match_type_guard 为既有裸指针 API，经 as_ptr 桥接。
        if let Some(tg) = match_type_guard(op, binop.left.as_ptr(), binop.right.as_ptr()) {
          // if (auto tgtLocal = tg->target->as<AstExprLocal>()) { ... }
          // `tg.target()` 是 TypeGuard 记录的存活裸指针字段：局部句柄折叠判空。
          let target = OptNode::from_ptr(tg.target());
          if let Some(tgt_local) = target.try_as::<AstExprLocal>() {
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
            // arena 句柄经 `get_mut` 物化本帧独占借用（arena_handle 模块契约）。
            let arena = &mut self.allocator.get_mut().refinement_arena;
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
            // arena 句柄经 `get_mut` 物化本帧独占借用（arena_handle 模块契约）。
            return Some(
              self
                .allocator
                .get_mut()
                .refinement_arena
                .conjunction_mut(l, r),
            );
          }
          return if l_ref.is_some() { l_ref } else { r_ref };
        } else if op == AstExprBinaryOp::Or {
          // (A or B) truthy => at least one truthy; an unrefined side means we can't narrow.
          if let (Some(l), Some(r)) = (l_ref, r_ref) {
            // arena 句柄经 `get_mut` 物化本帧独占借用（arena_handle 模块契约）。
            return Some(
              self
                .allocator
                .get_mut()
                .refinement_arena
                .disjunction_mut(l, r),
            );
          }
        }
        // falls through to `return std::nullopt;`
        return None;
      }

      // else if (auto unop = condition->as<AstExprUnary>()) { ... }
      if let Some(unop) = ast_node_try_as::<AstExprUnary>(expr) {
        // if (unop->op == AstExprUnary::Not)
        if unop.op == AstExprUnaryOp::Not {
          // if (auto inner = resolveCondition(unop->expr))
          //     return arena.negation(*inner);
          // expr 已句柄化；resolve_condition 为既有裸指针 API，经 as_ptr 桥接。
          if let Some(inner) = self.resolve_condition(unop.expr.as_ptr()) {
            // arena 句柄经 `get_mut` 物化本帧独占借用（arena_handle 模块契约）。
            return Some(
              self
                .allocator
                .get_mut()
                .refinement_arena
                .negation_mut(inner),
            );
          }
        }
      }

      // return std::nullopt;
      None
    }
  }
}
