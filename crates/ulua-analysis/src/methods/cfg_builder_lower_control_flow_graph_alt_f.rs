//! Source: `Analysis/src/ControlFlowGraph.cpp:310-345` (hand-ported)
//! C++ `void CFGBuilder::lower(AstStatWhile* statWhile)`.
use alloc::string::ToString;

use ulua_ast::records::ast_stat_while::AstStatWhile;

use crate::{
  enums::block_kind::BlockKind,
  records::{block::Block, cfg_builder::CfgBuilder},
  type_aliases::refinement_id_control_flow_graph::RefinementId,
};

impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `stat_while` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn lower_ast_stat_while(&mut self, stat_while: *mut AstStatWhile) {
    unsafe {
      // Block* preLoop = currentBlock.get();
      let pre_loop: *mut Block = self.current_block;

      // Loop Header — receives the back-edge so we don't seal it yet. Resolve the
      // condition inside the Header's scope so reads of variables mutated in the
      // body hit this unsealed block, emit an incomplete Join, and get their
      // operands filled in when the Header is sealed after the back-edge.
      // Block* loopHeader = newBlock(BlockKind::Condition, "while-loop condition", preLoop);
      let loop_header = self.new_block(
        BlockKind::Condition,
        "while-loop condition".to_string(),
        pre_loop,
      );
      // std::optional<RefinementId> ref;
      // { BlockScope scope(*this, loopHeader); ref = resolveCondition(statWhile->condition); }
      let ref_opt: Option<RefinementId> = {
        let saved = self.current_block;
        self.block_scope_cfg_builder_block(loop_header);
        let r = self.resolve_condition((*stat_while).condition);
        self.current_block = saved; // ~BlockScope restores currentBlock
        r
      };

      // Block* bodyBlock = newBlock(BlockKind::Linear, "while-loop body", loopHeader);
      let body_block = self.new_block(
        BlockKind::Linear,
        "while-loop body".to_string(),
        loop_header,
      );
      // if (ref) emitRefineInstruction(bodyBlock, *ref);
      if let Some(r) = ref_opt {
        self.emit_refine_instruction(body_block, r);
      }
      // seal(bodyBlock);
      self.seal(body_block);
      // Block* bodyExit;
      // { BlockScope scope(*this, bodyBlock); lower(statWhile->body); bodyExit = currentBlock.get(); }
      let body_exit: *mut Block = {
        let saved = self.current_block;
        self.block_scope_cfg_builder_block(body_block);
        self.lower_ast_stat_block((*stat_while).body);
        let exit = self.current_block;
        self.current_block = saved; // ~BlockScope restores currentBlock
        exit
      };

      // You can seal the loop Header now because no predecessors will be added to it.
      // bodyExit->addSuccessor(loopHeader);
      (*body_exit).add_successor(loop_header);
      // seal(loopHeader);
      self.seal(loop_header);

      // Block* exitBlock = newBlock(BlockKind::Linear, "while-loop exit", loopHeader);
      let exit_block = self.new_block(
        BlockKind::Linear,
        "while-loop exit".to_string(),
        loop_header,
      );
      // if (ref) emitRefineInstruction(exitBlock, allocator->refinementArena.negation(*ref));
      if let Some(r) = ref_opt {
        let neg = (*self.allocator).refinement_arena.negation_mut(r);
        self.emit_refine_instruction(exit_block, neg);
      }
      // seal(exitBlock);
      self.seal(exit_block);
      // currentBlock = NotNull{exitBlock};
      self.current_block = exit_block;
    }
  }
}
