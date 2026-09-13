mod _inner {
  #[inline(always)]
  pub const fn luau_insn_aux_a(aux: u32) -> u32 {
    aux & 0xff
  }
}
pub use _inner::{luau_insn_aux_a, luau_insn_aux_a as LUAU_INSN_AUX_A};
