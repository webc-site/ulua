use ulua_vm::records::{call_info::CallInfo, lua_state::lua_State};

use crate::{
  enums::address_kind_a_64::AddressKindA64,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
  },
};
const X0: RegisterA64 = RegisterA64 { bits: 2 };
const X1: RegisterA64 = RegisterA64 { bits: (1 << 3) | 2 };
const X19: RegisterA64 = RegisterA64 {
  bits: (19 << 3) | 2,
};
const X24: RegisterA64 = RegisterA64 {
  bits: (24 << 3) | 2,
};

pub fn emit_update_pc_for_exit_assembly_builder_a_64(build: &mut AssemblyBuilderA64) {
  let r_state = X19;
  let r_code = X24;
  let x0 = X0;
  let x1 = X1;

  build.add_register_a_64_register_a_64_register_a_64_i32(x0, r_code, x0, 0);
  build.ldr(
    x1,
    AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
      r_state,
      core::mem::offset_of!(lua_State, ci) as i32,
      AddressKindA64::Imm,
    ),
  );
  build.str(
    x0,
    AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
      x1,
      core::mem::offset_of!(CallInfo, savedpc) as i32,
      AddressKindA64::Imm,
    ),
  );
}
