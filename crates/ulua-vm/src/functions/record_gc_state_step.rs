#[cfg(feature = "luai_gcmetrics")]
use ulua_common::macros::luau_assert::LUAU_ASSERT;

#[cfg(feature = "luai_gcmetrics")]
use crate::records::global_state::global_State;

pub const GCSPAUSE: i32 = 0;
pub const GCSPROPAGATE: i32 = 1;
pub const GCSPROPAGATEAGAIN: i32 = 2;
pub const GCSATOMIC: i32 = 3;
pub const GCSSWEEP: i32 = 4;

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
