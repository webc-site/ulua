use ulua_vm::records::lua_state::lua_State;

use crate::{
  enums::size_x_64::SizeX64,
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

pub fn emit_update_base(build: &mut AssemblyBuilderX64) {
  // rBase = l->base. In this crate the stack base register is `rbp` (see luau_reg helpers).
  build.mov(
    OperandX64::reg(RegisterX64::RBP),
    OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      r_state(),
      core::mem::offset_of!(lua_State, base) as i32,
    ),
  );
}
