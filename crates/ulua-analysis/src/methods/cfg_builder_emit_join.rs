use crate::{
  methods::block_set_reaching_definition::block_set_reaching_definition,
  records::{
    block_registry::resolve_block_mut, cfg_builder::CfgBuilder, join::Join, symbol::Symbol,
  },
  type_aliases::{block_id::BlockId, instr_id::InstrId},
};

impl CfgBuilder {
  /// `Join* CFGBuilder::emitJoin(Block* block, Symbol sym)`.
  /// Reference: `ControlFlowGraph.cpp:258-265`. `block`/返回值自 #17 续起为
  /// `BlockId`/`InstrId` u32 句柄（Join 变体在补全侧经注册表甄别）。
  pub(crate) fn emit_join(&mut self, block: BlockId, sym: Symbol) -> InstrId {
    let def = self.new_definition(sym.clone());
    let j = self.emit::<Join, _>(block, def);
    let block_ref =
      resolve_block_mut(block).expect("BlockId 为本次构建期 register_block 发放的存活句柄");
    block_set_reaching_definition(block_ref, sym, def);
    self.incomplete_joins.get_or_insert(block).insert(j);
    j
  }
}
