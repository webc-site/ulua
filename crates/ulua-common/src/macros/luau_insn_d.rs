mod _inner {
  #[inline(always)]
  pub const fn luau_insn_d(insn: u32) -> i32 {
    (insn as i32) >> 16
  }
}
pub use _inner::{luau_insn_d, luau_insn_d as LUAU_INSN_D};

// Macro shim: C++ LUAU_INSN_D is a #define; translated callers use both
// `luau_insn_d(x)` (the const fn above) and `luau_insn_d!(x)` forms.
#[macro_export]
macro_rules! __luau_insn_d_shim {
  ($insn:expr) => {
    $crate::macros::luau_insn_d::luau_insn_d($insn)
  };
}
// Rename-re-export carries only the macro namespace, so it can share the
// module with the same-named const fn.
pub use __luau_insn_d_shim as LUAU_INSN_D;
pub use __luau_insn_d_shim as luau_insn_d;
