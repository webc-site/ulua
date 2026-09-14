use ulua_vm::enums::tms::TMS;

use crate::{
  enums::size_x_64::SizeX64,
  functions::{
    emit_update_base_emit_common_x_64::emit_update_base, luau_reg_address::luau_reg_address,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_data::K_INVALID_INST_IDX, ir_op::IrOp, ir_reg_alloc_x_64::IrRegAllocX64,
    native_context::NativeContext, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

const fn r_state() -> RegisterX64 {
  RegisterX64 {
    bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Qword as u8,
  }
}

const fn r_native_context() -> RegisterX64 {
  RegisterX64 {
    bits: (13u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Qword as u8,
  }
}

pub fn call_arith_helper(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  ra: i32,
  b: OperandX64,
  c: OperandX64,
  tm: TMS,
) {
  let mut call_wrap =
    IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(regs, build, K_INVALID_INST_IDX);

  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    OperandX64::reg(r_state()),
    IrOp::new(),
  );
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    luau_reg_address(ra),
    IrOp::new(),
  );
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(SizeX64::Qword, b, IrOp::new());
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(SizeX64::Qword, c, IrOp::new());

  let off = match tm {
    TMS::TmAdd => core::mem::offset_of!(NativeContext, lua_v_doarithadd),
    TMS::TmSub => core::mem::offset_of!(NativeContext, lua_v_doarithsub),
    TMS::TmMul => core::mem::offset_of!(NativeContext, lua_v_doarithmul),
    TMS::TmDiv => core::mem::offset_of!(NativeContext, lua_v_doarithdiv),
    TMS::TmIDiv => core::mem::offset_of!(NativeContext, lua_v_doarithidiv),
    TMS::TmMod => core::mem::offset_of!(NativeContext, lua_v_doarithmod),
    TMS::TmPow => core::mem::offset_of!(NativeContext, lua_v_doarithpow),
    TMS::TmUnm => core::mem::offset_of!(NativeContext, lua_v_doarithunm),
    _ => {
      CODEGEN_ASSERT!(false); // "Invalid doarith helper operation tag"
      0
    }
  } as i32;

  call_wrap.call(&OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    r_native_context(),
    off,
  ));

  emit_update_base(build);
}
