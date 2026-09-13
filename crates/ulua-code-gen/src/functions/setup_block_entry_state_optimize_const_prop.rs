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

fn luau_bytecode_type_optional_bit() -> u8 {
  LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0 as u8
}

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

  for i in 0..type_info.argument_types.len() {
    let et = type_info.argument_types[i];
    let tag = et & !luau_bytecode_type_optional_bit();

    if tag == LuauBytecodeType::LBC_TYPE_ANY.0 as u8
      || (et & luau_bytecode_type_optional_bit()) != 0
    {
      continue;
    }

    let cfg: &CfgInfo = &function.cfg;
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

  let state_ptr = state as *mut ConstPropState;
  let build_ptr = build as *mut IrBuilder;
  propagate_tags_from_predecessors(
    function,
    block,
    Box::new(move |i: usize| unsafe { (*state_ptr).regs[i].tag }),
    Box::new(move |i: usize, tag: u8| unsafe {
      let op = (*build_ptr).vm_reg(i as u8);
      (*state_ptr).update_tag(op, tag);
    }),
  );
}
