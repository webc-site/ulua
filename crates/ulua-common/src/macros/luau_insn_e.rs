mod _inner {
  #[inline]
  pub const fn luau_insn_e(insn: u32) -> i32 {
    (insn as i32) >> 8
  }
}
pub use _inner::luau_insn_e;
