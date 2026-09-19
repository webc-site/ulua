use core::mem::offset_of;

use crate::{
  enums::kind_a_64::KindA64,
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, native_context::NativeContext,
    register_a_64::RegisterA64,
  },
  type_aliases::mem::mem,
};

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
  RegisterA64 {
    bits: kind as u8 | (index << 3),
  }
}

const X0: RegisterA64 = reg(KindA64::X, 0);
const X1: RegisterA64 = reg(KindA64::X, 1);
const R_NATIVE_CONTEXT: RegisterA64 = reg(KindA64::X, 20);

pub fn emit_exit_assembly_builder_a_64_bool(build: &mut AssemblyBuilderA64, continue_in_vm: bool) {
  build.mov_register_a_64_i32(X0, continue_in_vm as i32);
  build.ldr(
    X1,
    mem(
      R_NATIVE_CONTEXT,
      offset_of!(NativeContext, gate_exit) as i32,
    ),
  );
  build.br(X1);
}
