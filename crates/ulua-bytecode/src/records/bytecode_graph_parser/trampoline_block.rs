use ulua_common::{
  enums::luau_opcode::LuauOpcode, functions::get_jump_target::get_jump_target,
  records::instruction::Instruction,
};

use super::BytecodeGraphParser;
use crate::records::{bc_imm::BcImm, bc_op::BcOp};
// ── abs-r139：并自 `methods/bytecode_graph_parser_is_jump_trampoline.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn is_jump_trampoline(&self, pc: u32, code: &[Instruction]) -> bool {
    let pc = pc as usize;
    if pc >= code.len() || code[pc].opcode() != Some(LuauOpcode::LOP_JUMP) {
      return false;
    }

    if pc + 1 >= code.len() {
      return false;
    }

    if code[pc + 1].opcode() != Some(LuauOpcode::LOP_JUMPX) {
      return false;
    }

    if pc + 2 >= code.len() {
      return false;
    }

    let target = get_jump_target(code[pc + 2].raw(), (pc + 2) as u32) as u32;
    target == (pc + 1) as u32
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_make_block.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn make_block(&mut self, pc: u32) -> BcOp {
    let new_block_op = self.func.add_block();
    *self.block_by_pc.get_or_insert(pc) = new_block_op;
    let new_block = self.func.block_op(new_block_op);
    new_block.sortkey = pc;
    new_block_op
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_push_imm_input.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// cpp `addImmInput` 三个重载（对应 `BytecodeGraphParser.h:402-427`）的单一源：
  /// 每个立即数操作数独占一条_imm 记录、索引取 `len - 1`，**不按值复用**——
  /// 复用会让同一指令的多个操作数共享记录，`setFbSlot` 之类改写连带污染另一个。
  /// 变体（即 kind）与载荷随 `imm` 参数由各入口注入（`BcImm` 为带载荷 enum）。
  /// 追加与索引包装委托 `BcFunction::add_imm_value` 单源。
  pub(crate) fn push_imm_input(&mut self, inst: BcOp, imm: BcImm) {
    let op = self.func.add_imm_value(imm);
    self.func.inst_op(inst).ops.push_back(op);
  }

  /// cpp `addImmInput(BcRef<BcInst>, bool)`（`BytecodeGraphParser.h:402-409`）：
  /// Boolean 入口，独占记录不复用的理由见 `push_imm_input`（此前的 `position()`
  /// 去重会让同一指令的多个 bool 操作数共享记录，改写其一时连带污染另一个）。
  pub(crate) fn add_imm_input_bc_inst_bool(&mut self, inst: BcOp, value: bool) {
    self.push_imm_input(inst, BcImm::Boolean(value));
  }

  /// cpp `addImmInput(BcRef<BcInst>, int32_t)`（`BytecodeGraphParser.h:411-418`）：
  /// Int 入口，独占记录不复用的理由见 `push_imm_input`（同一指令的 paramCount /
  /// returnCount / fbSlot 可能取值相同，复用会让后续改写连带改掉另一个操作数）。
  pub(crate) fn add_imm_input_bc_inst_i32(&mut self, inst: BcOp, value: i32) {
    self.push_imm_input(inst, BcImm::Int(value));
  }

  /// cpp `addImmInput(BcRef<BcInst>, uint32_t)`（`BytecodeGraphParser.h:420-427`）：
  /// Import 型入口，独占记录不复用的理由见 `push_imm_input`。
  pub(crate) fn add_imm_input_bc_inst_u32(&mut self, inst: BcOp, value: u32) {
    self.push_imm_input(inst, BcImm::Import(value));
  }
}
