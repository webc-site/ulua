use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_UNREACHABLE;

use crate::{
  enums::{ir::IrValueKind, ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind},
  functions::{
    get_live_in_out_value_count::get_live_in_out_value_count,
    kill_ir_utils::kill_ir_function_ir_block_at,
    propagate_tags_from_predecessors::propagate_tags_from_predecessors,
    visit_vm_reg_defs_uses_ir_visit_use_def::visit_vm_reg_defs_uses_t_ir_function_ir_block,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_state::RegisterSet, block_vm_reg_live_in_computation::BlockVmRegLiveInComputation,
    const_prop_state::ConstPropState, ir_block::IrBlock, ir_function::IrFunction,
    remove_dead_store_state::RemoveDeadStoreState,
  },
};

pub fn get_reload_offset(kind: IrValueKind) -> i32 {
  match kind {
    IrValueKind::Unknown | IrValueKind::None | IrValueKind::Float | IrValueKind::Count => {
      CODEGEN_ASSERT!(false, "Invalid operand restore value kind");
    }
    IrValueKind::Tag => {
      return 8;
    }
    IrValueKind::Int | IrValueKind::Int64 | IrValueKind::Pointer | IrValueKind::Double => {
      return 0;
    }
    IrValueKind::Tvalue => {
      return 0;
    }
  }

  CODEGEN_ASSERT!(false, "Invalid operand restore value kind");
  LUAU_UNREACHABLE!();
}

/// 起始块以索引传入，与链式统计入口保持一致。
pub fn get_live_out_value_count(function: &mut IrFunction, block_idx: u32) -> u32 {
  get_live_in_out_value_count(function, block_idx, false).1
}

/// cpp `isUsedInVmExitSync` (CodeGen/src/IrAnalysis.cpp:140-153):
/// 判断 `target_inst_idx` 是否被 `inst_idx` 处的 VM exit sync 用作参数
pub fn is_used_in_vm_exit_sync(function: &IrFunction, inst_idx: u32, target_inst_idx: u32) -> bool {
  if let Some(sync_info) = function.vm_exit_info.find(&inst_idx) {
    for arg_op in sync_info.arg_ops.as_slice() {
      CODEGEN_ASSERT!(arg_op.kind() == IrOpKind::Inst);

      if arg_op.index() == target_inst_idx {
        return true;
      }
    }
  }

  false
}

/// 目标块以索引传入。
pub fn setup_block_entry_state_ir_function_ir_block_remove_dead_store_state(
  function: &IrFunction,
  block_idx: u32,
  state: &mut RemoveDeadStoreState,
) {
  propagate_tags_from_predecessors(function, block_idx, state);
}

/// 目标块以索引传入，免去向 function 反查块号。
pub fn save_block_exit_state(
  function: &mut IrFunction,
  block_idx: u32,
  state: &mut ConstPropState,
) {
  let mut tags: Vec<u8> = Vec::with_capacity((state.max_reg as usize) + 1);

  for item in state.regs.iter().take(state.max_reg as usize + 1) {
    tags.push(item.tag);
  }

  function.block_exit_tags[block_idx as usize] = tags;
}

pub fn compute_block_live_in_reg_set(
  function: &mut IrFunction,
  block: &IrBlock,
  def_rs: &mut RegisterSet,
  captured_regs: &mut [u64; 4],
) -> RegisterSet {
  let mut visitor = BlockVmRegLiveInComputation::new(def_rs, captured_regs);
  visit_vm_reg_defs_uses_t_ir_function_ir_block(&mut visitor, function, block);
  visitor.in_rs
}

pub fn kill_unused_blocks(function: &mut IrFunction) {
  // 从 1 开始，因为第 0 个 block 是 entry block
  // kill 只就地标记块为 Dead（不改 blocks 长度），故顺序游标即等价于定长区间遍历
  for i in 1..function.blocks.len() {
    let block = &function.blocks[i];
    if block.kind != IrBlockKind::Dead && block.use_count == 0 {
      kill_ir_function_ir_block_at(function, i);
    }
  }
}

// VmReg 寄存器位图公共辅助：收口各处重复的 `(regs[r / 64] & (1u64 << (r % 64)))`
// 位测试/置位表达式。`regs` 为 64 位字序的位图（RegisterSet.regs 为 4×u64，共 256 位）。

/// 位图每字位数（编译期常量，来自 u64::BITS）
const REG_WORD_BITS: usize = u64::BITS as usize;

/// 测试 `reg` 位是否置位
#[inline]
pub fn reg_bit_test(regs: &[u64], reg: usize) -> bool {
  regs[reg / REG_WORD_BITS] & (1u64 << (reg % REG_WORD_BITS)) != 0
}

/// 置位 `reg` 位
#[inline]
pub fn reg_bit_set(regs: &mut [u64], reg: usize) {
  regs[reg / REG_WORD_BITS] |= 1u64 << (reg % REG_WORD_BITS);
}
