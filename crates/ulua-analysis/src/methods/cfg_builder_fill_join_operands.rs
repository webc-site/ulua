//! Source: `Analysis/src/ControlFlowGraph.cpp:488-497` (hand-ported)
//! C++ `void CFGBuilder::fillJoinOperands(Block* block, Join* j)`.
use alloc::vec::Vec;

use crate::{
  records::{
    block_registry::resolve_block,
    cfg_builder::CfgBuilder,
    instr_registry::{resolve_instruction, resolve_instruction_mut},
    join::Join,
    sym_def_registry::resolve_sym_def,
  },
  type_aliases::{block_id::BlockId, instr_id::InstrId, instruction::InstructionMember},
};

/// 指令句柄 → `Join` 只读视图：`j` 由 `emit_join` 经 `emit::<Join, _>` 构造，
/// 变体甄别必命中（`IntoInstruction<Join>` 一一构造 `Instruction::Join`）；
/// 空哨兵/越界句柄解析失败即 `instr_registry` 契约被破坏，expect 比 cpp 的
/// 悬垂 `Join*` 解引用保守。
pub(crate) fn join_of_instr<'a>(j: InstrId) -> &'a Join {
  <Join as InstructionMember>::get_if(
    resolve_instruction(j).expect("InstrId 为本次构建期 register_instruction 发放的存活句柄"),
  )
  .expect("emit::<Join> 构造 Join 变体，get_if::<Join> 必命中")
}

/// 指令句柄 → `Join` 可变视图（seal 期操作数补全的唯一写回点，借用纪律见
/// `instr_registry` 模块契约）。
fn join_of_instr_mut<'a>(j: InstrId) -> &'a mut Join {
  <Join as InstructionMember>::get_if_mut(
    resolve_instruction_mut(j).expect("InstrId 为本次构建期 register_instruction 发放的存活句柄"),
  )
  .expect("emit::<Join> 构造 Join 变体，get_if_mut::<Join> 必命中")
}

impl CfgBuilder {
  /// `block`/`j` 自 #17 续起为 u32 句柄：前驱经 `block_registry`、Join 读写经
  /// `instr_registry` 解析（见各模块契约），构建期单分析线程、builder 独占
  /// `&mut self`，写回点顺序借用不重叠。
  /// 对应 C++ `void CFGBuilder::fillJoinOperands(Block* block, Join* j)` (`cpp/Analysis/src/ControlFlowGraph.cpp:601`)。
  pub fn fill_join_operands(&mut self, block: BlockId, j: InstrId) {
    // C++:
    //   for (BlockId pred : block->getPredecessors()) {
    //       auto def = readVariable(pred, j->definition->sym);
    //       j->operands.emplace_back(def);
    //   }
    //   trimTrivialJoin(j);
    // Snapshot predecessors: readVariable recurses and may mutate `block`.
    let preds: Vec<BlockId> = resolve_block(block)
      .expect("BlockId 为本次构建期 register_block 发放的存活句柄")
      .get_predecessors()
      .clone();
    // j->definition 是 emitJoin 里 new_definition 刚发放的存活句柄（NotNull
    // 语义），经注册表解析读 sym，与 cpp 的 `j->definition->sym` 同一步骤。
    let sym = resolve_sym_def(join_of_instr(j).definition)
      .expect("Join.definition 为构建期 register_sym_def 发放的存活句柄")
      .sym
      .clone();
    for pred in preds {
      let def = self.read_variable(pred, sym.clone());
      join_of_instr_mut(j).operands.push(def);
    }
    self.trim_trivial_join(j);
  }
}
