use core::mem::offset_of;

use ulua_vm::records::lua_state::LuaState;

use crate::{
  enums::size_x_64::SizeX64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, emit_common_x_64::R_STATE, operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

// C++ EmitCommonX64.h: `constexpr RegisterX64 rState = r15;`
pub fn emit_update_base(build: &mut AssemblyBuilderX64) {
  // rBase = l->base。cpp EmitCommonX64.h: `inline constexpr RegisterX64 rBase = r14;`
  build.mov(
    OperandX64::reg(RegisterX64::R14),
    OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      R_STATE,
      offset_of!(LuaState, base) as i32,
    ),
  );
}
