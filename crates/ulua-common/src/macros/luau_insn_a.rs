mod _inner {
  #[inline]
  pub const fn luau_insn_a(insn: u32) -> u32 {
    (insn >> 8) & 0xff
  }
}
pub use _inner::{luau_insn_a, luau_insn_a as LUAU_INSN_A};

// Macro shim: C++ LUAU_INSN_A is a #define; translated callers use both
// `luau_insn_a(x)` (the const fn above) and `luau_insn_a!(x)` forms.
#[macro_export]
macro_rules! __luau_insn_a_shim {
  ($insn:expr) => {
    $crate::macros::luau_insn_a::luau_insn_a($insn)
  };
}
// Rename-re-export carries only the macro namespace, so it can share the
// module with the same-named const fn.
pub use __luau_insn_a_shim as LUAU_INSN_A;
pub use __luau_insn_a_shim as luau_insn_a;
