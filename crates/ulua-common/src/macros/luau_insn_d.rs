mod _inner {
  #[inline]
  pub const fn luau_insn_d(insn: u32) -> i32 {
    (insn as i32) >> 16
  }
}
pub use _inner::luau_insn_d;
