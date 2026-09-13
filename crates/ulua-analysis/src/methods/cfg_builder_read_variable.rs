//! Source: `Analysis/src/ControlFlowGraph.cpp:464-486` (hand-ported)
//! C++ `DefId CFGBuilder::readVariable(BlockId block, Symbol sym)`.
use crate::{
  records::{cfg_builder::CfgBuilder, symbol::Symbol},
  type_aliases::{block_id::BlockId, def_id_control_flow_graph::DefId},
};

impl CfgBuilder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn read_variable(&mut self, block: BlockId, sym: Symbol) -> DefId {
    unsafe {
      // C++:
      //   if (auto v = block->getReachingDefinition(sym); v != nullptr)
      //       return NotNull{v};
      let v = (*block).get_reaching_definition(sym.clone());
      if !v.is_null() {
        return v;
      }

      // if (!isSealed(block)) { Join* j = emitJoin(block, sym); return j->definition; }
      if !self.is_sealed(block) {
        let j = self.emit_join(block, sym);
        (*j).definition
      }
      // else if (block->getPredecessors().size() == 1) {
      //     auto def = readVariable(block->getPredecessors()[0], sym);
      //     block->setReachingDefinition(sym, def);
      //     return def;
      // }
      else if (*block).get_predecessors().len() == 1 {
        let pred = (*block).get_predecessors()[0];
        let def = self.read_variable(pred, sym.clone());
        (*block).set_reaching_definition(sym, def);
        def
      }
      // else { Join* j = emitJoin(block, sym); fillJoinOperands(block, j); return j->definition; }
      else {
        let j = self.emit_join(block, sym);
        self.fill_join_operands(block, j);
        (*j).definition
      }
    }
  }
}
