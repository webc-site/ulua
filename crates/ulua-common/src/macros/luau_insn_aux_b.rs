mod _inner {
  #[inline(always)]
  pub const fn luau_insn_aux_b(aux: u32) -> u32 {
    (aux >> 8) & 0xff
  }
}
pub use _inner::luau_insn_aux_b;
