use crate::{
  enums::{address_kind_a_64::AddressKindA64, kind_a_64::KindA64},
  functions::{
    emit_add_offset::emit_add_offset, emit_update_base_emit_common_a_64::emit_update_base,
  },
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
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
const X3: RegisterA64 = reg(KindA64::X, 3);
const X4: RegisterA64 = reg(KindA64::X, 4);
const R_STATE: RegisterA64 = reg(KindA64::X, 19);
const R_NATIVE_CONTEXT: RegisterA64 = reg(KindA64::X, 20);
const R_BASE: RegisterA64 = reg(KindA64::X, 22);
const R_CONSTANTS: RegisterA64 = reg(KindA64::X, 23);
const R_CODE: RegisterA64 = reg(KindA64::X, 24);

const SIZEOF_INSTRUCTION: usize = 4;

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
  AddressA64 {
    kind: AddressKindA64::Imm,
    base,
    offset: RegisterA64::NOREG,
    data,
  }
}

pub fn emit_fallback_assembly_builder_a_64_i32_i32(
  build: &mut AssemblyBuilderA64,
  offset: i32,
  pcpos: i32,
) {
  build.mov_register_a_64_register_a_64(X0, R_STATE);
  emit_add_offset(build, X1, R_CODE, pcpos as usize * SIZEOF_INSTRUCTION);
  build.mov_register_a_64_register_a_64(X2, R_BASE);
  build.mov_register_a_64_register_a_64(X3, R_CONSTANTS);
  build.ldr(X4, mem(R_NATIVE_CONTEXT, offset));
  build.blr(X4);

  emit_update_base(build);
}
