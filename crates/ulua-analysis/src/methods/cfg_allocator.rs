//! `cfg_allocator` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::String;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::block_kind::BlockKind,
  records::{
    block::Block, block_registry::register_block, cfg_allocator::CfgAllocator,
    instr_registry::register_instruction, sym_def::SymDef, sym_def_registry::register_sym_def,
    symbol::Symbol,
  },
  type_aliases::{
    block_id::BlockId, def_id_control_flow_graph::DefId, instr_id::InstrId,
    instruction::Instruction,
  },
};

// Source: `Analysis/src/ControlFlowGraph.cpp:112-118` (hand-ported)
// C++ `void CFGAllocator::freeze()`.

impl CfgAllocator {
  pub fn freeze(&mut self) {
    // C++:
    //   block.freeze();
    //   defs.freeze();
    //   refinementArena.freeze();
    //   frozen = true;
    self.block.freeze();
    self.defs.freeze();
    // `RefinementArena::freeze()` == `allocator.freeze()` over its
    // `TypedAllocator<Refinement>` (field is `pub(crate)`, same crate).
    self.refinement_arena.allocator.freeze();
    self.frozen = true;
  }
}

// Source: `Analysis/src/ControlFlowGraph.cpp:102-105` (hand-ported)
// C++ `Block* CFGAllocator::newBlock(BlockKind kind, std::string debugName)`.

impl CfgAllocator {
  pub fn new_block(&mut self, kind: BlockKind, debug_name: String) -> BlockId {
    // C++: return block.allocate(kind, debugName);
    // TypedAllocator::allocate takes the constructed value; build the Block
    // in place (C++ constructs `T{args...}` inside allocate).
    // 裸指针仅在注册点出现：arena 地址经 `register_block` 换成 u32 句柄，
    // 之后全链（preds/succs、builder 集合、转储）只持有/比较句柄
    // （见 `records::block_registry` 模块契约）。
    register_block(self.block.allocate(Block::new(kind, debug_name)))
  }
}

// Source: `Analysis/src/ControlFlowGraph.cpp:107-110` (hand-ported)
// C++ `DefId CFGAllocator::newDefinition(Symbol sym, size_t version)`.

impl CfgAllocator {
  pub fn new_definition(&mut self, sym: Symbol, version: usize) -> DefId {
    // C++: return NotNull{defs.allocate(SymDef{sym, version})};
    // 裸指针仅在注册点出现：arena 地址经 `register_sym_def` 换成 u32 句柄，
    // 之后全链（指令操作数、reaching definitions、use-def、refinement 命题）
    // 只持有/比较句柄（见 `records::sym_def_registry` 模块契约）。
    register_sym_def(self.defs.allocate(SymDef::new(sym, version)))
  }
}

// Source: `Analysis/include/Luau/ControlFlowGraph.h:237-243` (hand-ported)
// C++ `template<typename T, typename... Args> InstrId CFGAllocator::newInstruction(Args&&... args)`.

impl CfgAllocator {
  /// C++ builds `Instruction* inst = instructions.allocate(T{args...})` and
  /// returns `NotNull{inst}`. The variant `T{args...}` is constructed by the
  /// caller (`CFGBuilder::emit`) and threaded through as the `Instruction`.
  pub fn new_instruction(&mut self, inst: Instruction) -> InstrId {
    // C++: LUAU_ASSERT(!frozen);
    LUAU_ASSERT!(!self.frozen);
    // C++: Instruction* inst = instructions.allocate(...); return NotNull{inst};
    // 裸指针仅在注册点出现：arena 地址经 `register_instruction` 换成 u32
    // 句柄，之后全链（Block.instructions、incomplete_joins、转储）只持有/
    // 比较句柄（见 `records::instr_registry` 模块契约）。
    register_instruction(self.instructions.allocate(inst))
  }
}
