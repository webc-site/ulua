use ulua_common::{
  enums::luau_capture_type::LuauCaptureType,
  macros::{luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B},
};

use crate::{
  enums::ir_cmd::IrCmd, macros::codegen_assert::CODEGEN_ASSERT, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_capture(build: &mut IrBuilder, pc: *const Instruction, _pcpos: i32) {
  let type_ = LUAU_INSN_A(unsafe { *pc }) as u8;
  let index = LUAU_INSN_B(unsafe { *pc }) as u8;

  match type_ {
    x if x == LuauCaptureType::LCT_VAL as u8 => {
      let reg = build.vm_reg(index as u8);
      let const_uint = build.const_uint(0);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::CAPTURE, reg, const_uint);
    }
    x if x == LuauCaptureType::LCT_REF as u8 => {
      let reg = build.vm_reg(index as u8);
      let const_uint = build.const_uint(1);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::CAPTURE, reg, const_uint);
    }
    x if x == LuauCaptureType::LCT_UPVAL as u8 => {
      let upvalue = build.vm_upvalue(index as u8);
      let const_uint = build.const_uint(0);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::CAPTURE, upvalue, const_uint);
    }
    _ => {
      CODEGEN_ASSERT!(false, "Unknown upvalue capture type");
    }
  }
}
