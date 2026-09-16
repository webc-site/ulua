use core::mem::offset_of;

use ulua_vm::records::{
  closure::{Closure, LClosure},
  proto::Proto,
};

use crate::{
  enums::kind_a_64::KindA64,
  macros::call_fallback_yield::CALL_FALLBACK_YIELD,
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, module_helpers::ModuleHelpers,
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
const X2: RegisterA64 = reg(KindA64::X, 2);
const R_CONSTANTS: RegisterA64 = reg(KindA64::X, 23);
const R_CODE: RegisterA64 = reg(KindA64::X, 24);
const R_CLOSURE: RegisterA64 = reg(KindA64::X, 25);

pub fn emit_continue_call(build: &mut AssemblyBuilderA64, helpers: &mut ModuleHelpers) {
  crate::CODEGEN_ASSERT!(CALL_FALLBACK_YIELD == 1);

  build.tbnz(X0, 0, &mut helpers.exit_no_continue_vm);

  build.ldr(
    X1,
    mem(
      X0,
      (offset_of!(Closure, inner) + offset_of!(LClosure, p)) as i32,
    ),
  );

  build.ldr(X2, mem(X1, offset_of!(Proto, exectarget) as i32));
  build.cbz(X2, &mut helpers.exit_continue_vm);

  build.mov_register_a_64_register_a_64(R_CLOSURE, X0);
  build.ldp(R_CONSTANTS, R_CODE, mem(X1, offset_of!(Proto, k) as i32));
  build.br(X2);
}
