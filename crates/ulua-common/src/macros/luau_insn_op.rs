mod _inner {
  #[inline]
  pub const fn luau_insn_op(insn: u32) -> u32 {
    insn & 0xff
  }
}
pub use _inner::luau_insn_op;
