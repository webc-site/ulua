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
//
// r15-v1 保留注记（w6d 逐点定性）：本体 14 处场域读点（13 处 gcmetrics 计时/
// 记账 + 1 处 gcstate 判别读）皆经函数参数域句柄进入——`g` 的来源是调用方
// 透传的裸指针而非状态开场字段的局部别名，属票面「函数参数来源」定性保留类，
// 不入 gs_ref 门面迁移面。本函数自身即调用簇的字段写收口体（单 unsafe 体+
// 契约形制，与 nn_alias 判例同格），无散点裸解引用可收，故亦不另造句柄。
#[cfg(feature = "luai_gcmetrics")]
pub(crate) unsafe fn record_gc_state_step(
  g: *mut global_State,
  startgcstate: i32,
  seconds: f64,
  assist: bool,
  work: usize,
) {
  // SAFETY: 契约保证 `g` 存活且 startgcstate 与当前 gcstate 语义配对，块内仅累加计时/记账字段
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
