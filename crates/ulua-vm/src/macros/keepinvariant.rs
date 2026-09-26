use crate::{
  macros::{
    gc_satomic::GCSATOMIC,
    gc_spropagate::{GCSPROPAGATE, GCSPROPAGATEAGAIN},
  },
  records::global_state::global_State,
};

/// # Safety
///
/// `g` must point to a valid `global_State`.
#[inline(always)]
pub(crate) unsafe fn keepinvariant(g: *const global_State) -> bool {
  // Safety: 契约保证 `g` 指向存活 `global_State`，此处仅读 `gcstate` 字段
  unsafe {
    let gcstate = (*g).gcstate as i32;
    matches!(gcstate, GCSPROPAGATE | GCSPROPAGATEAGAIN | GCSATOMIC)
  }
}
