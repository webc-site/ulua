mod _inner {
  #[inline(always)]
  pub const fn luau_insn_aux_kb(aux: u32) -> u32 {
    aux & 0x1
  }
}
pub use _inner::{luau_insn_aux_kb, luau_insn_aux_kb as LUAU_INSN_AUX_KB};
