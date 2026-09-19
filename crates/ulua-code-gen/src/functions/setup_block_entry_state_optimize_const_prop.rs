use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

use crate::{
  functions::{
    propagate_tags_from_predecessors::propagate_tags_from_predecessors,
    try_get_luau_tag_for_bc_type::try_get_luau_tag_for_bc_type,
  },
  records::{
    bytecode_type_info::BytecodeTypeInfo, cfg_info::CfgInfo, const_prop_state::ConstPropState,
    ir_block::IrBlock, ir_builder::IrBuilder, ir_function::IrFunction,
  },
};

const LBC_TYPE_OPTIONAL_BIT: u8 = LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0 as u8;

pub fn setup_block_entry_state_ir_builder_ir_function_ir_block_const_prop_state(
  build: &mut IrBuilder,
  function: &mut IrFunction,
  block: &IrBlock,
  state: &mut ConstPropState,
) {
  let block_flags = block.flags;
  let entry_arg_check_bit = 1u8 << 2;
  if (block_flags & entry_arg_check_bit) != 0 {
    return;
  }

  let type_info: &BytecodeTypeInfo = &function.bc_original_type_info;
  let cfg: &CfgInfo = &function.cfg;

  for (i, &et) in type_info.argument_types.iter().enumerate() {
    let tag = et & !LBC_TYPE_OPTIONAL_BIT;

    if tag == LuauBytecodeType::LBC_TYPE_ANY.0 as u8 || (et & LBC_TYPE_OPTIONAL_BIT) != 0 {
      continue;
    }

    if (cfg.written.regs[i / 64] & (1u64 << (i % 64))) != 0 {
      continue;
    }

    if cfg.written.vararg_seq && i >= cfg.written.vararg_start as usize {
      continue;
    }

    if (cfg.captured.regs[i / 64] & (1u64 << (i % 64))) != 0 {
      continue;
    }

    if let Some(vm_tag) = try_get_luau_tag_for_bc_type(tag, /* ignore_optional_part */ true) {
      let op = build.vm_reg(i as u8);
      state.update_tag(op, vm_tag);
    }
  }

  propagate_tags_from_predecessors(function, block, state);
}
