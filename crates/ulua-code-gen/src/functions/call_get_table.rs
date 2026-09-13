use crate::{
  enums::size_x_64::SizeX64,
  functions::{
    emit_update_base_emit_common_x_64::emit_update_base, luau_reg_address::luau_reg_address,
  },
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

pub fn call_get_table(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  rb: i32,
  c: OperandX64,
  ra: i32,
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
    luau_reg_address(rb),
    IrOp::new(),
  );
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(SizeX64::Qword, c, IrOp::new());
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    luau_reg_address(ra),
    IrOp::new(),
  );

  call_wrap.call(&OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    r_native_context(),
    core::mem::offset_of!(NativeContext, lua_v_gettable) as i32,
  ));

  emit_update_base(build);
}
