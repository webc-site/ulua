use crate::{
  methods::block_set_reaching_definition::block_set_reaching_definition,
  records::{block::Block, cfg_builder::CfgBuilder, join::Join, symbol::Symbol},
};

impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `block` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// `Join* CFGBuilder::emitJoin(Block* block, Symbol sym)`.
  /// Reference: `ControlFlowGraph.cpp:258-265`.
  pub unsafe fn emit_join(&mut self, block: *mut Block, sym: Symbol) -> *mut Join {
    let def = self.new_definition(sym.clone());
    let j: *mut Join = self.emit::<Join, _>(block, def);
    let block_ref = unsafe { &mut *block };
    block_set_reaching_definition(block_ref, sym, def);
    self.incomplete_joins.get_or_insert(block).insert(j);
    j
  }
}
