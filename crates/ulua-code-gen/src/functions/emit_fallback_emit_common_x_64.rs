use crate::{
  enums::size_x_64::SizeX64,
  functions::emit_update_base_emit_common_x_64::emit_update_base,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_data::K_INVALID_INST_IDX, ir_op::IrOp, ir_reg_alloc_x_64::IrRegAllocX64,
    operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

const fn r_state() -> RegisterX64 {
  RegisterX64 {
    bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Qword as u8,
  }
}

// C++ EmitCommonX64.h: `constexpr RegisterX64 rConstants = r12;`
const fn r_constants() -> RegisterX64 {
  RegisterX64 {
    bits: (12u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Qword as u8,
  }
}

// C++ EmitCommonX64.h: `constexpr RegisterX64 rNativeContext = r13;`
const fn r_native_context() -> RegisterX64 {
  RegisterX64 {
    bits: (13u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Qword as u8,
  }
}

const K_STACK_OFFSET_TO_LOCALS: i32 = 16 + 32;

fn s_code() -> OperandX64 {
  OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    RegisterX64::RSP,
    K_STACK_OFFSET_TO_LOCALS + 8,
  )
}

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
    OperandX64::reg(r_state()),
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

  // rBase = rbp in this crate.
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    OperandX64::reg(RegisterX64::RBP),
    IrOp::new(),
  );
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    OperandX64::reg(r_constants()),
    IrOp::new(),
  );

  call_wrap.call(&OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    r_native_context(),
    offset,
  ));

  emit_update_base(build);
}
