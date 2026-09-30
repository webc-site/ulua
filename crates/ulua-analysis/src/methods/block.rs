//! `block` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::{
  records::{block::Block, block_registry::resolve_block_mut, symbol::Symbol},
  type_aliases::{block_id::BlockId, def_id_control_flow_graph::DefId},
};

impl BlockId {
  /// 对应 C++ `void Block::addSuccessor(BlockId target)`
  /// (`cpp/Analysis/src/ControlFlowGraph.cpp:65`)：target 记入 self 的后继、
  /// self 记入 target 的前驱。
  ///
  /// 两段 `resolve_block_mut` 顺序借用、互不重叠（先结束 self 的可变借用再
  /// 取 target 的），自环（`self == target`）亦然——与迁移前
  /// `self->successors.emplace_back(target); target->predecessors.emplace_back(this);`
  /// 的裸指针写回形态逐位同构（见 `block_registry` 模块契约）。
  pub(crate) fn add_successor(self, target: BlockId) {
    // C++: successors.emplace_back(target);
    resolve_block_mut(self)
      .expect("BlockId 为本次构建期 register_block 发放的存活句柄")
      .successors
      .push(target);
    // C++: target->predecessors.emplace_back(this);
    resolve_block_mut(target)
      .expect("BlockId 为本次构建期 register_block 发放的存活句柄")
      .predecessors
      .push(self);
  }
}

pub fn block_set_reaching_definition(block: &mut Block, sym: Symbol, def: DefId) {
  *block.reaching_definitions.get_or_insert(sym) = def;
}
