use core::mem::offset_of;

use ulua_vm::{
  macros::lua_callinfo_native::LUA_CALLINFO_NATIVE,
  records::{call_info::CallInfo, lua_state::LuaState},
};

use crate::{
  enums::size_x_64::SizeX64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, emit_common_x_64::R_STATE, operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

// C++ EmitCommonX64.h: `constexpr RegisterX64 rState = r15;`
pub fn emit_clear_native_flag(build: &mut AssemblyBuilderX64) {
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      R_STATE,
      offset_of!(LuaState, ci) as i32,
    ),
  );
  build.and_(
    OperandX64::mem(
      SizeX64::Dword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RAX,
      offset_of!(CallInfo, flags) as i32,
    ),
    OperandX64::imm(!LUA_CALLINFO_NATIVE),
  );
}
