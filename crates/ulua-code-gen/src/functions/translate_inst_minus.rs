use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  macros::{luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B},
};
use ulua_vm::enums::{lua_type::LuaType, tms::TMS};

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    get_initialized_fallback::get_initialized_fallback,
    is_userdata_bytecode_type::is_userdata_bytecode_type,
  },
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder, ir_op::IrOp},
  type_aliases::instruction_ir_translation::Instruction,
};
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_minus(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let bc_types = build.function.get_bytecode_types_at(pcpos);

  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
  let rb = LUAU_INSN_B(unsafe { *pc }) as u8;

  if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8 {
    let reg_rb = build.vm_reg(rb);
    let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
    let const_tag_vector = build.const_tag(LuaType::Vector as u8);
    let vm_exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, const_tag_vector, vm_exit);

    let reg_rb = build.vm_reg(rb);
    let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_rb);
    let va = build.inst_ir_cmd_ir_op(IrCmd::UnmVec, vb);
    let va = build.inst_ir_cmd_ir_op(IrCmd::TagVector, va);
    let reg_ra = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, va);
    return;
  }

  if is_userdata_bytecode_type(bc_types.a) {
    let savedpc_arg = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let const_int_unm = build.const_int(TMS::TmUnm as i32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::DoArith,
      reg_ra,
      reg_rb,
      reg_rb,
      const_int_unm,
    );
    return;
  }

  let mut fallback = IrOp::new();

  let reg_rb = build.vm_reg(rb);
  let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
  let const_tag_number = build.const_tag(LuaType::Number as u8);
  let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
    build.vm_exit(pcpos as u32)
  } else {
    get_initialized_fallback(build, &mut fallback, pcpos)
  };
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, const_tag_number, exit_or_fallback);

  let reg_rb = build.vm_reg(rb);
  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_rb);
  let va = build.inst_ir_cmd_ir_op(IrCmd::UnmNum, vb);

  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, reg_ra, va);

  if ra != rb {
    let reg_ra = build.vm_reg(ra);
    let const_tag_number = build.const_tag(LuaType::Number as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg_ra, const_tag_number);
  }

  if fallback.kind() != IrOpKind::None {
    let next = build.block_at_inst((pcpos + 1) as u32);
    let scope = FallbackStreamScope::new(build, fallback, next);
    let build = &mut *scope.build;

    let savedpc_arg = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let const_int_unm = build.const_int(TMS::TmUnm as i32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::DoArith,
      reg_ra,
      reg_rb,
      reg_rb,
      const_int_unm,
    );
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
  }
}
