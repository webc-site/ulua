use core::mem::offset_of;

use ulua_vm::records::lua_state::LuaState;

use crate::{
  enums::kind_a_64::KindA64,
  records::{
    assembly_builder_a_64::AssemblyBuilderA64,
    register_a_64::{RegisterA64, reg},
  },
  type_aliases::mem::mem,
};

const R_STATE: RegisterA64 = reg(KindA64::X, 19);
const R_BASE: RegisterA64 = reg(KindA64::X, 25);

pub fn emit_update_base(build: &mut AssemblyBuilderA64) {
  build.ldr(R_BASE, mem(R_STATE, offset_of!(LuaState, base) as i32));
}
