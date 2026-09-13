use crate::{
  enums::size_x_64::SizeX64,
  functions::dword_reg::dword_reg,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, native_context::NativeContext,
    operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

// C++ EmitCommonX64.h: `constexpr RegisterX64 rNativeContext = r13;`
const fn r_native_context() -> RegisterX64 {
  RegisterX64 {
    bits: (13u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Qword as u8,
  }
}

pub fn emit_exit(build: &mut AssemblyBuilderX64, continue_in_vm: bool) {
  let eax = dword_reg(RegisterX64::RAX);

  if continue_in_vm {
    build.mov(OperandX64::reg(eax), OperandX64::imm(1));
  } else {
    build.xor_(OperandX64::reg(eax), OperandX64::reg(eax));
  }

  build.jmp_operand_x_64(OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    r_native_context(),
    core::mem::offset_of!(NativeContext, gate_exit) as i32,
  ));
}
