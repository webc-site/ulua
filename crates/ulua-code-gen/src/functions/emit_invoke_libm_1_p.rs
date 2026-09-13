use core::mem::size_of;

use ulua_vm::type_aliases::t_value::TValue;

use crate::{
  enums::{address_kind_a_64::AddressKindA64, kind_a_64::KindA64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
  },
};

const K_MAX_IMMEDIATE: usize = (1 << 12) - 1;
const K_TEMP_SLOTS: u32 = 1;
const S_TEMPORARY_DATA: i32 = 9 * 8;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
  RegisterA64 {
    bits: kind as u8 | (index << 3),
  }
}

const X0: RegisterA64 = reg(KindA64::X, 0);
const X1: RegisterA64 = reg(KindA64::X, 1);
const X20: RegisterA64 = reg(KindA64::X, 20);
const X25: RegisterA64 = reg(KindA64::X, 25);
const SP: RegisterA64 = reg(KindA64::None, 31);
const D0: RegisterA64 = reg(KindA64::D, 0);

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
  AddressA64 {
    kind: AddressKindA64::Imm,
    base,
    offset: RegisterA64::NOREG,
    data,
  }
}

pub fn emit_invoke_libm_1_p(build: &mut AssemblyBuilderA64, func: usize, arg: i32) {
  CODEGEN_ASSERT!(K_TEMP_SLOTS >= 1);
  CODEGEN_ASSERT!(S_TEMPORARY_DATA as usize <= K_MAX_IMMEDIATE);

  let tvalue_size = size_of::<TValue>() as i32;
  let value_offset = core::mem::offset_of!(TValue, value) as i32;

  build.ldr(D0, mem(X25, arg * tvalue_size + value_offset));
  build.add_register_a_64_register_a_64_u16(X0, SP, S_TEMPORARY_DATA as u16);
  build.ldr(X1, mem(X20, func as i32));
  build.blr(X1);
}
