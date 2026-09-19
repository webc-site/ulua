mod _inner {
  #[inline(always)]
  pub const fn luau_insn_aux_slot(aux: u32) -> u32 {
    aux >> 16
  }
}
pub use _inner::luau_insn_aux_slot;
