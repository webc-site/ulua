mod _inner {
  #[inline(always)]
  pub const fn luau_insn_aux_not(aux: u32) -> u32 {
    aux >> 31
  }
}
pub use _inner::{luau_insn_aux_not, luau_insn_aux_not as LUAU_INSN_AUX_NOT};
