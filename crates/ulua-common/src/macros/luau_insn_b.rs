mod _inner {
  #[inline]
  pub const fn luau_insn_b(insn: u32) -> u32 {
    (insn >> 16) & 0xff
  }
}
pub use _inner::{luau_insn_b, luau_insn_b as LUAU_INSN_B};

// Macro shim: C++ LUAU_INSN_B is a #define; translated callers use both
// `luau_insn_b(x)` (the const fn above) and `luau_insn_b!(x)` forms.
#[macro_export]
macro_rules! __luau_insn_b_shim {
  ($insn:expr) => {
    $crate::macros::luau_insn_b::luau_insn_b($insn)
  };
}
// Rename-re-export carries only the macro namespace, so it can share the
// module with the same-named const fn.
pub use __luau_insn_b_shim as LUAU_INSN_B;
pub use __luau_insn_b_shim as luau_insn_b;
