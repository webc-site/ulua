use crate::{
  enums::size_x_64::SizeX64,
  functions::{emit_update_base_emit_common_x_64::emit_update_base, s_code::s_code},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64,
    emit_common_x_64::{R_CONSTANTS, R_NATIVE_CONTEXT, R_STATE},
    ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_data::K_INVALID_INST_IDX,
    ir_op::IrOp,
    ir_reg_alloc_x_64::IrRegAllocX64,
    operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

// C++ EmitCommonX64.h: `constexpr RegisterX64 rConstants = r12;`
// C++ EmitCommonX64.h: `constexpr RegisterX64 rNativeContext = r13;`
const SIZEOF_INSTRUCTION: i32 = 4;

pub fn emit_fallback(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  offset: i32,
  pcpos: i32,
) {
  // fallback(l, instruction, base, k)
  let mut call_wrap =
    IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(regs, build, K_INVALID_INST_IDX);

  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    OperandX64::reg(R_STATE),
    IrOp::new(),
  );

  let reg = call_wrap.suggest_next_argument_register(SizeX64::Qword);
  build.mov(OperandX64::reg(reg), s_code());
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    OperandX64::mem(
      SizeX64::None,
      RegisterX64::NOREG,
      1,
      reg,
      pcpos * SIZEOF_INSTRUCTION,
    ),
    IrOp::new(),
  );

  // cpp EmitCommonX64.h: `inline constexpr RegisterX64 rBase = r14;`
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    OperandX64::reg(RegisterX64::R14),
    IrOp::new(),
  );
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    OperandX64::reg(R_CONSTANTS),
    IrOp::new(),
  );

  call_wrap.call(&OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    R_NATIVE_CONTEXT,
    offset,
  ));

  emit_update_base(build);
}
