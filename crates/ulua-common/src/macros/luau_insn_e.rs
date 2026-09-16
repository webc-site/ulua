mod _inner {
  #[inline(always)]
  pub const fn luau_insn_e(insn: u32) -> i32 {
    (insn as i32) >> 8
  }
}
pub use _inner::{luau_insn_e, luau_insn_e as LUAU_INSN_E};

// Macro shim: C++ LUAU_INSN_E is a #define; translated callers use both
// `luau_insn_e(x)` (the const fn above) and `luau_insn_e!(x)` forms.
#[macro_export]
macro_rules! __luau_insn_e_shim {
  ($insn:expr) => {
    $crate::macros::luau_insn_e::luau_insn_e($insn)
  };
}
// Rename-re-export carries only the macro namespace, so it can share the
// module with the same-named const fn.
pub use __luau_insn_e_shim as LUAU_INSN_E;
pub use __luau_insn_e_shim as luau_insn_e;
