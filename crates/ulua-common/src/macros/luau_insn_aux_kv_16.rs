mod _inner {
  #[inline(always)]
  pub const fn luau_insn_aux_kv16(aux: u32) -> u32 {
    aux & 0xffff
  }
}
pub use _inner::{luau_insn_aux_kv16, luau_insn_aux_kv16 as LUAU_INSN_AUX_KV16};
