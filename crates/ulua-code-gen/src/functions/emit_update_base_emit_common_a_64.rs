use core::mem::offset_of;

use ulua_vm::records::lua_state::lua_State;

use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
  type_aliases::mem::mem,
};

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
  RegisterA64 {
    bits: kind as u8 | (index << 3),
  }
}

const R_STATE: RegisterA64 = reg(KindA64::X, 19);
const R_BASE: RegisterA64 = reg(KindA64::X, 22);

pub fn emit_update_base(build: &mut AssemblyBuilderA64) {
  build.ldr(R_BASE, mem(R_STATE, offset_of!(lua_State, base) as i32));
}
