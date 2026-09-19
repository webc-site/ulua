mod _inner {
  #[inline]
  pub const fn luau_insn_c(insn: u32) -> u32 {
    (insn >> 24) & 0xff
  }
}
pub use _inner::luau_insn_c;
