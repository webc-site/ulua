use core::mem::offset_of;

use ulua_vm::records::{call_info::CallInfo, lua_state::lua_State};

use crate::{
  enums::size_x_64::SizeX64,
  functions::s_code::s_code,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

// C++ EmitCommonX64.h: `constexpr RegisterX64 rState = r15;`
const fn r_state() -> RegisterX64 {
  RegisterX64 {
    bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Qword as u8,
  }
}

pub fn emit_update_pc_for_exit(build: &mut AssemblyBuilderX64) {
  // edx = pcpos * sizeof(Instruction)
  build.add(OperandX64::reg(RegisterX64::RDX), s_code());
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      r_state(),
      offset_of!(lua_State, ci) as i32,
    ),
  );
  build.mov(
    OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RAX,
      offset_of!(CallInfo, savedpc) as i32,
    ),
    OperandX64::reg(RegisterX64::RDX),
  );
}
