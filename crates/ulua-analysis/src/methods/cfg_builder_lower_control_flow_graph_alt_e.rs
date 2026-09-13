//! Source: `Analysis/src/ControlFlowGraph.cpp:267-307` (hand-ported)
//! C++ `void CFGBuilder::lower(AstStatIf* statIf)`.
use alloc::string::ToString;
use core::ptr::null_mut;

use ulua_ast::records::ast_stat_if::AstStatIf;

use crate::{
  enums::block_kind::BlockKind,
  records::{block::Block, cfg_builder::CfgBuilder},
};
impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `stat_if` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn lower_ast_stat_if(&mut self, stat_if: *mut AstStatIf) {
    unsafe {
      // Block* currBlock = currentBlock.get();
      let curr_block: *mut Block = self.current_block;

      // Block* thenBlock = newBlock(BlockKind::Linear, "then branch", currBlock);
      let then_block = self.new_block(BlockKind::Linear, "then branch".to_string(), curr_block);
      // auto ref = resolveCondition(statIf->condition);
      let ref_opt = self.resolve_condition((*stat_if).condition);
      // if (ref) emitRefineInstruction(thenBlock, *ref);
      if let Some(r) = ref_opt {
        self.emit_refine_instruction(then_block, r);
      }

      // Then only has one predecessor
      // seal(thenBlock);
      self.seal(then_block);
      // Block* thenExit;
      // { BlockScope scope(*this, thenBlock); lower(statIf->thenbody); thenExit = currentBlock.get(); }
      let then_exit: *mut Block = {
        let saved = self.current_block;
        self.block_scope_cfg_builder_block(then_block);
        self.lower_ast_stat_block((*stat_if).thenbody);
        let exit = self.current_block;
        self.current_block = saved; // ~BlockScope restores currentBlock
        exit
      };

      // Else branch (may be nullptr, another AstStatIf for elseif, or a block)
      // Block* elseBlock = newBlock(BlockKind::Linear, "else branch", currBlock);
      let else_block = self.new_block(BlockKind::Linear, "else branch".to_string(), curr_block);
      // Block* elseExit = elseBlock; // If there is an else body, overwrite this
      let mut else_exit: *mut Block = else_block;
      // seal(elseBlock);
      self.seal(else_block);
      // if (ref) emitRefineInstruction(elseBlock, allocator->refinementArena.negation(*ref));
      if let Some(r) = ref_opt {
        let neg = (*self.allocator).refinement_arena.negation_mut(r);
        self.emit_refine_instruction(else_block, neg);
      }

      // if (statIf->elsebody) { BlockScope scope(*this, elseBlock); lower(statIf->elsebody); elseExit = currentBlock.get(); }
      if !(*stat_if).elsebody.is_null() {
        let saved = self.current_block;
        self.block_scope_cfg_builder_block(else_block);
        self.lower_ast_stat((*stat_if).elsebody);
        else_exit = self.current_block;
        self.current_block = saved; // ~BlockScope restores currentBlock
      }

      // Merge block — all paths converge here
      // Block* mergeBlock = newBlock(BlockKind::Linear, "merge");
      let merge_block = self.new_block(BlockKind::Linear, "merge".to_string(), null_mut());
      // thenExit->addSuccessor(mergeBlock);
      (*then_exit).add_successor(merge_block);
      // elseExit->addSuccessor(mergeBlock);
      (*else_exit).add_successor(merge_block);
      // seal(mergeBlock);
      self.seal(merge_block);
      // currentBlock = NotNull{mergeBlock};
      self.current_block = merge_block;
    }
  }
}
