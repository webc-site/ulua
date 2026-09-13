use ulua_vm::records::{
  call_info::CallInfo, global_state::global_State, lua_callbacks::LuaCallbacks,
  lua_state::lua_State,
};

use crate::{
  enums::{address_kind_a_64::AddressKindA64, kind_a_64::KindA64},
  functions::{
    emit_exit_code_gen_a_64::emit_exit_assembly_builder_a_64_bool,
    emit_update_base_emit_common_a_64::emit_update_base,
  },
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, label::Label,
    register_a_64::RegisterA64,
  },
};

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
  RegisterA64 {
    bits: kind as u8 | (index << 3),
  }
}

const X0: RegisterA64 = reg(KindA64::X, 0);
const X1: RegisterA64 = reg(KindA64::X, 1);
const X2: RegisterA64 = reg(KindA64::X, 2);
const W0: RegisterA64 = reg(KindA64::W, 0);
const W1: RegisterA64 = reg(KindA64::W, 1);
const R_STATE: RegisterA64 = reg(KindA64::X, 19);
const R_BASE: RegisterA64 = reg(KindA64::X, 22);
const R_CODE: RegisterA64 = reg(KindA64::X, 24);

const SIZEOF_INSTRUCTION: u16 = 4;

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
  AddressA64 {
    kind: AddressKindA64::Imm,
    base,
    offset: RegisterA64::NOREG,
    data,
  }
}

pub fn emit_interrupt(build: &mut AssemblyBuilderA64) {
  let mut skip = Label::default();

  build.mov_register_a_64_register_a_64(R_BASE, X1);

  build.ldr(
    X2,
    mem(R_STATE, core::mem::offset_of!(lua_State, global) as i32),
  );
  build.ldr(
    X2,
    mem(
      X2,
      (core::mem::offset_of!(global_State, cb) + core::mem::offset_of!(LuaCallbacks, interrupt))
        as i32,
    ),
  );
  build.cbz(X2, &mut skip);

  build.add_register_a_64_register_a_64_register_a_64_i32(X0, R_CODE, X0, 0);
  build.ldr(
    X1,
    mem(R_STATE, core::mem::offset_of!(lua_State, ci) as i32),
  );
  build.str(X0, mem(X1, core::mem::offset_of!(CallInfo, savedpc) as i32));

  build.mov_register_a_64_register_a_64(X0, R_STATE);
  build.mov_register_a_64_i32(W1, -1);
  build.blr(X2);

  build.ldrb(
    W0,
    mem(R_STATE, core::mem::offset_of!(lua_State, status) as i32),
  );
  build.cbz(W0, &mut skip);

  build.ldr(
    X1,
    mem(R_STATE, core::mem::offset_of!(lua_State, ci) as i32),
  );
  build.ldr(X0, mem(X1, core::mem::offset_of!(CallInfo, savedpc) as i32));
  build.sub_register_a_64_register_a_64_u16(X0, X0, SIZEOF_INSTRUCTION);
  build.str(X0, mem(X1, core::mem::offset_of!(CallInfo, savedpc) as i32));

  emit_exit_assembly_builder_a_64_bool(build, false);

  build.set_label_label(&mut skip);

  build.mov_register_a_64_register_a_64(X0, R_BASE);
  emit_update_base(build);
  build.br(X0);
}
