mod _inner {
  #[inline]
  pub const fn luau_insn_op(insn: u32) -> u32 {
    insn & 0xff
  }
}
pub use _inner::{luau_insn_op, luau_insn_op as LUAU_INSN_OP};

// Macro shim: C++ LUAU_INSN_OP is a #define; translated callers use both
// `luau_insn_op(x)` (the const fn above) and `luau_insn_op!(x)` forms.
#[macro_export]
macro_rules! __luau_insn_op_shim {
  ($insn:expr) => {
    $crate::macros::luau_insn_op::luau_insn_op($insn)
  };
}
// Rename-re-export carries only the macro namespace, so it can share the
// module with the same-named const fn.
pub use __luau_insn_op_shim as LUAU_INSN_OP;
pub use __luau_insn_op_shim as luau_insn_op;
