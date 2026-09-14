use core::mem::size_of;

use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  macros::{luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B, luau_insn_c::LUAU_INSN_C},
};
use ulua_vm::{enums::lua_type::LuaType, type_aliases::t_value::TValue};

use crate::{
  enums::ir_cmd::IrCmd,
  functions::is_userdata_bytecode_type::is_userdata_bytecode_type,
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder},
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_get_table_n(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
) {
  let insn = unsafe { *pc };
  let ra = LUAU_INSN_A(insn) as u8;
  let rb = LUAU_INSN_B(insn) as u8;
  let c = LUAU_INSN_C(insn) as u8;

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  if is_userdata_bytecode_type(bc_types.a) {
    let savedpc_arg = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let c_plus_one = build.const_uint((c + 1) as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetTable, reg_ra, reg_rb, c_plus_one);
    return;
  }

  let fallback = build.fallback_block(pcpos as u32);

  let reg_rb = build.vm_reg(rb);
  let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
  let const_tag_table = build.const_tag(LuaType::Table as u8);
  let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8 {
    build.vm_exit(pcpos as u32)
  } else {
    fallback
  };
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, const_tag_table, exit_or_fallback);

  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);

  let c_op = build.const_int(c as i32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, vb, c_op, fallback);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, vb, fallback);

  let zero = build.const_int(0);
  let arr_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, vb, zero);

  let c_times_sizeof = build.const_int((c as i32) * (size_of::<TValue>() as i32));
  let arr_el_tval = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, arr_el, c_times_sizeof);
  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, arr_el_tval);

  let next = build.block_at_inst((pcpos + 1) as u32);

  let scope = FallbackStreamScope::new(build, fallback, next);
  let build = &mut *scope.build;

  let savedpc_arg = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  let c_plus_one = build.const_uint((c + 1) as u32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetTable, reg_ra, reg_rb, c_plus_one);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
