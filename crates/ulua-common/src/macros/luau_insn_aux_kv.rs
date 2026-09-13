mod _inner {
  #[inline(always)]
  pub const fn luau_insn_aux_kv(aux: u32) -> u32 {
    aux & 0xffffff
  }
}
pub use _inner::{luau_insn_aux_kv, luau_insn_aux_kv as LUAU_INSN_AUX_KV};
