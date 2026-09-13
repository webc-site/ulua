use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::size_x_64::SizeX64,
  functions::{luau_reg_tag::luau_reg_tag, luau_reg_value::luau_reg_value},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_data::K_INVALID_INST_IDX, ir_op::IrOp, ir_reg_alloc_x_64::IrRegAllocX64,
    native_context::NativeContext, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

const fn reg(index: u8, size: SizeX64) -> RegisterX64 {
  RegisterX64 {
    bits: (index << RegisterX64::INDEX_SHIFT) | size as u8,
  }
}

const R_NATIVE_CONTEXT: RegisterX64 = reg(13, SizeX64::Qword);
const XMM0: RegisterX64 = reg(0, SizeX64::Xmmword);

fn s_temporary_slot() -> OperandX64 {
  OperandX64::mem(SizeX64::Qword, RegisterX64::NOREG, 1, RegisterX64::RSP, 0)
}

pub fn emit_builtin_math_frexp(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  ra: i32,
  arg: i32,
  nresults: i32,
) {
  let mut call_wrap =
    IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(regs, build, K_INVALID_INST_IDX);
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Xmmword,
    luau_reg_value(arg),
    IrOp::new(),
  );
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    s_temporary_slot(),
    IrOp::new(),
  );
  call_wrap.call(&OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    R_NATIVE_CONTEXT,
    core::mem::offset_of!(NativeContext, libm_frexp) as i32,
  ));

  build.vmovsd_operand_x_64_operand_x_64(luau_reg_value(ra), OperandX64::reg(XMM0));
  build.mov(luau_reg_tag(ra), OperandX64::imm(LuaType::Number as i32));

  if nresults > 1 {
    build.vcvtsi2sd(
      OperandX64::reg(XMM0),
      OperandX64::reg(XMM0),
      OperandX64::mem(SizeX64::Dword, RegisterX64::NOREG, 1, RegisterX64::RSP, 0),
    );
    build.vmovsd_operand_x_64_operand_x_64(luau_reg_value(ra + 1), OperandX64::reg(XMM0));
    build.mov(
      luau_reg_tag(ra + 1),
      OperandX64::imm(LuaType::Number as i32),
    );
  }
}
