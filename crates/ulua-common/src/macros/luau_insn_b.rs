mod _inner {
  #[inline]
  pub const fn luau_insn_b(insn: u32) -> u32 {
    (insn >> 16) & 0xff
  }
}
pub use _inner::luau_insn_b;
