use alloc::{string::String, vec::Vec};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::block_kind::BlockKind,
  records::symbol::Symbol,
  type_aliases::{block_id::BlockId, def_id_control_flow_graph::DefId, instr_id::InstrId},
};
#[derive(Debug, Clone)]
pub struct Block {
  pub kind: BlockKind,
  pub debug_name: String,
  pub(crate) instructions: Vec<InstrId>,
  pub(crate) predecessors: Vec<BlockId>,
  pub(crate) successors: Vec<BlockId>,
  pub(crate) reaching_definitions: DenseHashMap<Symbol, DefId>,
}

impl Block {
  /// `Block::Block(BlockKind kind, std::string debugName)`. Reference: `ControlFlowGraph.cpp:58-62`.
  pub fn new(kind: BlockKind, debug_name: String) -> Self {
    Self {
      kind,
      debug_name,
      instructions: Vec::new(),
      predecessors: Vec::new(),
      successors: Vec::new(),
      // `reachingDefinitions{Symbol{}}`.
      reaching_definitions: DenseHashMap::new(Symbol::default()),
    }
  }

  /// `const std::vector<InstrId>& Block::getInstructions() const`.
  pub fn get_instructions(&self) -> &Vec<InstrId> {
    &self.instructions
  }

  /// `const std::vector<BlockId>& Block::getPredecessors() const`.
  pub fn get_predecessors(&self) -> &Vec<BlockId> {
    &self.predecessors
  }

  /// `Definition* Block::getReachingDefinition(Symbol sym) const`. Reference: `ControlFlowGraph.cpp:75-80`.
  ///
  /// cpp 的 `nullptr` 缺省即「无定义」，Rust 侧收进 `Option<DefId>`（§2：不向
  /// 调用方泄漏魔法哨兵），句柄的解析统一走 `sym_def_registry::resolve_sym_def`。
  pub fn get_reaching_definition(&self, sym: Symbol) -> Option<DefId> {
    self.reaching_definitions.get(&sym).copied()
  }

  /// `const std::vector<BlockId>& Block::getSuccessors() const`.
  pub fn get_successors(&self) -> &Vec<BlockId> {
    &self.successors
  }

  /// `void Block::setReachingDefinition(Symbol sym, DefId def)`. Reference: `ControlFlowGraph.cpp:82-85`.
  pub fn set_reaching_definition(&mut self, sym: Symbol, def: DefId) {
    *self.reaching_definitions.get_or_insert(sym) = def;
  }
}
