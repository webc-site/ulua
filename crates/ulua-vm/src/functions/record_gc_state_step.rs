#[cfg(feature = "luai_gcmetrics")]
use ulua_common::macros::luau_assert::LUAU_ASSERT;

#[cfg(feature = "luai_gcmetrics")]
use crate::{
  macros::{
    gc_satomic::{GCSATOMIC, GCSSWEEP},
    gc_spause::GCSPAUSE,
    gc_spropagate::{GCSPROPAGATE, GCSPROPAGATEAGAIN},
  },
  records::global_state::global_State,
};

/// # Safety
///
/// `g` must point to a valid, properly initialized `global_State`.
#[cfg(feature = "luai_gcmetrics")]
pub(crate) unsafe fn record_gc_state_step(
  g: *mut global_State,
  startgcstate: i32,
  seconds: f64,
  assist: bool,
  work: usize,
) {
  // Safety: 契约保证 `g` 存活且 startgcstate 与当前 gcstate 语义配对，块内仅累加计时/记账字段
  unsafe {
    match startgcstate {
      GCSPAUSE => {
        if (*g).gcstate as i32 == GCSPROPAGATE {
          (*g).gcmetrics.currcycle.marktime += seconds;
          if assist {
            (*g).gcmetrics.currcycle.markassisttime += seconds;
          }
        }
      }
      GCSPROPAGATE | GCSPROPAGATEAGAIN => {
        (*g).gcmetrics.currcycle.marktime += seconds;
        (*g).gcmetrics.currcycle.markwork += work;
        if assist {
          (*g).gcmetrics.currcycle.markassisttime += seconds;
        }
      }
      GCSATOMIC => {
        (*g).gcmetrics.currcycle.atomictime += seconds;
      }
      GCSSWEEP => {
        (*g).gcmetrics.currcycle.sweeptime += seconds;
        (*g).gcmetrics.currcycle.sweepwork += work;
        if assist {
          (*g).gcmetrics.currcycle.sweepassisttime += seconds;
        }
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }

    if assist {
      (*g).gcmetrics.stepassisttimeacc += seconds;
      (*g).gcmetrics.currcycle.assistwork += work;
    } else {
      (*g).gcmetrics.stepexplicittimeacc += seconds;
      (*g).gcmetrics.currcycle.explicitwork += work;
    }
  }
}
