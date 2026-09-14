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
  unsafe {
    let gcstate = (*g).gcstate as i32;
    gcstate == GCSPROPAGATE || gcstate == GCSPROPAGATEAGAIN || gcstate == GCSATOMIC
  }
}
