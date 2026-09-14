use ulua_vm::{
  records::{
    call_info::CallInfo as CallInfoRecord, lua_state::lua_State as lua_StateRecord,
    lua_t_value::lua_TValue as TValueRecord, proto::Proto as ProtoRecord,
  },
  type_aliases::value::Value,
};

use crate::{
  enums::address_kind_a_64::AddressKindA64,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64,
    entry_locations_code_gen_a_64::EntryLocations, register_a_64::RegisterA64,
    unwind_builder::UnwindBuilder,
  },
};

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
  AddressA64 {
    kind: AddressKindA64::Imm,
    base,
    offset: RegisterA64::NOREG,
    data,
  }
}

trait UnwindBuilderExt {
  fn start_function(&mut self);
  fn prologue_a_64(&mut self, prologue_size: u32, stack_size: u32, regs: &[RegisterA64]);
  fn finish_function(&mut self, begin_offset: u32, end_offset: u32);
}

impl UnwindBuilderExt for UnwindBuilder {
  fn start_function(&mut self) {}
  fn prologue_a_64(&mut self, _prologue_size: u32, _stack_size: u32, _regs: &[RegisterA64]) {}
  fn finish_function(&mut self, _begin_offset: u32, _end_offset: u32) {}
}

const SP: RegisterA64 = RegisterA64 { bits: (31 << 3) };
const X0: RegisterA64 = RegisterA64 { bits: 2 };
const X1: RegisterA64 = RegisterA64 { bits: (1 << 3) | 2 };
const X2: RegisterA64 = RegisterA64 { bits: (2 << 3) | 2 };
const X3: RegisterA64 = RegisterA64 { bits: (3 << 3) | 2 };
const X9: RegisterA64 = RegisterA64 { bits: (9 << 3) | 2 };
const X19: RegisterA64 = RegisterA64 {
  bits: (19 << 3) | 2,
};
const X20: RegisterA64 = RegisterA64 {
  bits: (20 << 3) | 2,
};
const X21: RegisterA64 = RegisterA64 {
  bits: (21 << 3) | 2,
};
const X22: RegisterA64 = RegisterA64 {
  bits: (22 << 3) | 2,
};
const X23: RegisterA64 = RegisterA64 {
  bits: (23 << 3) | 2,
};
const X24: RegisterA64 = RegisterA64 {
  bits: (24 << 3) | 2,
};
const X25: RegisterA64 = RegisterA64 {
  bits: (25 << 3) | 2,
};
const X29: RegisterA64 = RegisterA64 {
  bits: (29 << 3) | 2,
};
const X30: RegisterA64 = RegisterA64 {
  bits: (30 << 3) | 2,
};

const K_STACK_SIZE: u16 = 128;

pub fn build_entry_function_assembly_builder_a_64_unwind_builder(
  build: &mut AssemblyBuilderA64,
  unwind: &mut UnwindBuilder,
) -> EntryLocations {
  let mut locations = EntryLocations {
    start: build.set_label(),
    ..Default::default()
  };

  build.sub_register_a_64_register_a_64_u16(SP, SP, K_STACK_SIZE);
  build.stp(X29, X30, mem(SP, 0));

  build.stp(X19, X20, mem(SP, 16));
  build.stp(X21, X22, mem(SP, 32));
  build.stp(X23, X24, mem(SP, 48));
  build.str(X25, mem(SP, 64));

  build.mov_register_a_64_register_a_64(X29, SP);

  locations.prologue_end = build.set_label();

  let prologue_size =
    build.get_label_offset(&locations.prologue_end) - build.get_label_offset(&locations.start);

  let r_state = X19;
  let r_native_context = X20;
  let r_global_state = X21;
  let r_base = X22;
  let r_constants = X23;
  let r_code = X24;
  let r_closure = X25;

  build.mov_register_a_64_register_a_64(r_state, X0);
  build.mov_register_a_64_register_a_64(r_native_context, X3);
  build.ldr(
    r_global_state,
    mem(X0, core::mem::offset_of!(lua_StateRecord, global) as i32),
  );
  build.ldr(
    r_base,
    mem(X0, core::mem::offset_of!(lua_StateRecord, base) as i32),
  );

  build.ldp(
    r_constants,
    r_code,
    mem(X1, core::mem::offset_of!(ProtoRecord, k) as i32),
  );

  build.ldr(
    X9,
    mem(X0, core::mem::offset_of!(lua_StateRecord, ci) as i32),
  );
  build.ldr(
    X9,
    mem(X9, core::mem::offset_of!(CallInfoRecord, func) as i32),
  );
  build.ldr(
    r_closure,
    mem(
      X9,
      core::mem::offset_of!(TValueRecord, value) as i32 + core::mem::offset_of!(Value, gc) as i32,
    ),
  );

  build.br(X2);

  locations.epilogue_start = build.set_label();

  build.ldr(X25, mem(SP, 64));
  build.ldp(X23, X24, mem(SP, 48));
  build.ldp(X21, X22, mem(SP, 32));
  build.ldp(X19, X20, mem(SP, 16));
  build.ldp(X29, X30, mem(SP, 0));
  build.add_register_a_64_register_a_64_u16(SP, SP, K_STACK_SIZE);

  build.ret();

  unwind.start_function();
  unwind.prologue_a_64(
    prologue_size,
    K_STACK_SIZE as u32,
    &[X29, X30, X19, X20, X21, X22, X23, X24, X25],
  );
  unwind.finish_function(build.get_label_offset(&locations.start), 0xffffffff);

  locations
}
