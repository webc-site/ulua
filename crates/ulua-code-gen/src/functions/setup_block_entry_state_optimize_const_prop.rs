use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::{
    propagate_tags_from_predecessors::propagate_tags_from_predecessors, reg_bitset::reg_bit_test,
    try_get_luau_tag_for_bc_type::try_get_luau_tag_for_bc_type,
  },
  records::{
    bytecode_type_info::BytecodeTypeInfo, cfg_info::CfgInfo, const_prop_state::ConstPropState,
    ir_block::K_BLOCK_FLAG_ENTRY_ARG_CHECK, ir_function::IrFunction, ir_op::IrOp,
  },
};

const LBC_TYPE_OPTIONAL_BIT: u8 = LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0 as u8;

/// 目标块以索引传入。
pub fn setup_block_entry_state_ir_builder_ir_function_ir_block_const_prop_state(
  function: &mut IrFunction,
  block_idx: u32,
  state: &mut ConstPropState,
) {
  let block_flags = function.blocks[block_idx as usize].flags;
  if (block_flags & K_BLOCK_FLAG_ENTRY_ARG_CHECK) != 0 {
    return;
  }

  let type_info: &BytecodeTypeInfo = &function.bc_original_type_info;
  let cfg: &CfgInfo = &function.cfg;

  for (i, &et) in type_info.argument_types.iter().enumerate() {
    let tag = et & !LBC_TYPE_OPTIONAL_BIT;

    if tag == LuauBytecodeType::LBC_TYPE_ANY.0 as u8 || (et & LBC_TYPE_OPTIONAL_BIT) != 0 {
      continue;
    }

    if reg_bit_test(&cfg.written.regs, i) {
      continue;
    }

    if cfg.written.vararg_seq && i >= cfg.written.vararg_start as usize {
      continue;
    }

    if reg_bit_test(&cfg.captured.regs, i) {
      continue;
    }

    if let Some(vm_tag) = try_get_luau_tag_for_bc_type(tag, /* ignore_optional_part */ true) {
      // cpp `build.vmReg(i)` 为纯构造函数（不触碰 builder 状态），就地内联为 IrOp
      let op = IrOp::ir_op_kind_u32(IrOpKind::VmReg, i as u32);
      state.update_tag(op, vm_tag);
    }
  }

  propagate_tags_from_predecessors(function, block_idx, state);
}
