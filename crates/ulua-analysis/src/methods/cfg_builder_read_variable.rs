//! Source: `Analysis/src/ControlFlowGraph.cpp:464-486` (hand-ported)
//! C++ `DefId CFGBuilder::readVariable(BlockId block, Symbol sym)`.
use crate::{
  methods::cfg_builder_fill_join_operands::join_of_instr,
  records::{
    block_registry::{resolve_block, resolve_block_mut},
    cfg_builder::CfgBuilder,
    symbol::Symbol,
  },
  type_aliases::{block_id::BlockId, def_id_control_flow_graph::DefId},
};

impl CfgBuilder {
  /// `block` 自 #17 续起为 u32 句柄（经 `block_registry` 解析，见该模块契约），
  /// Join 指令经 `instr_registry` 句柄流转，全函数无裸指针解引用。
  pub fn read_variable(&mut self, block: BlockId, sym: Symbol) -> DefId {
    // C++:
    //   if (auto v = block->getReachingDefinition(sym); v != nullptr)
    //       return NotNull{v};
    // cpp 的 `nullptr` 判据即「无定义」，Rust 侧收进 Option（§2），
    // 命中即构建期发放的存活句柄，直接返回。
    if let Some(v) = resolve_block(block)
      .expect("BlockId 为本次构建期 register_block 发放的存活句柄")
      .get_reaching_definition(sym.clone())
    {
      return v;
    }

    // if (!isSealed(block)) { Join* j = emitJoin(block, sym); return j->definition; }
    if !self.is_sealed(block) {
      let j = self.emit_join(block, sym);
      join_of_instr(j).definition
    }
    // else if (block->getPredecessors().size() == 1) {
    //     auto def = readVariable(block->getPredecessors()[0], sym);
    //     block->setReachingDefinition(sym, def);
    //     return def;
    // }
    else if resolve_block(block)
      .expect("BlockId 为本次构建期 register_block 发放的存活句柄")
      .get_predecessors()
      .len()
      == 1
    {
      let pred = resolve_block(block)
        .expect("BlockId 为本次构建期 register_block 发放的存活句柄")
        .get_predecessors()[0];
      let def = self.read_variable(pred, sym.clone());
      resolve_block_mut(block)
        .expect("BlockId 为本次构建期 register_block 发放的存活句柄")
        .set_reaching_definition(sym, def);
      def
    }
    // else { Join* j = emitJoin(block, sym); fillJoinOperands(block, j); return j->definition; }
    else {
      let j = self.emit_join(block, sym);
      self.fill_join_operands(block, j);
      join_of_instr(j).definition
    }
  }
}
