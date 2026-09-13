mod _inner {
  #[inline]
  pub const fn luau_insn_c(insn: u32) -> u32 {
    (insn >> 24) & 0xff
  }
}
pub use _inner::{luau_insn_c, luau_insn_c as LUAU_INSN_C};

// Macro shim: C++ LUAU_INSN_C is a #define; translated callers use both
// `luau_insn_c(x)` (the const fn above) and `luau_insn_c!(x)` forms.
#[macro_export]
macro_rules! __luau_insn_c_shim {
  ($insn:expr) => {
    $crate::macros::luau_insn_c::luau_insn_c($insn)
  };
}
// Rename-re-export carries only the macro namespace, so it can share the
// module with the same-named const fn.
pub use __luau_insn_c_shim as LUAU_INSN_C;
pub use __luau_insn_c_shim as luau_insn_c;
