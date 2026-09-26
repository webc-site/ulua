#[cfg(feature = "hard_mem_tests")]
use crate::functions::lua_c_validate::lua_c_validate;
#[cfg(feature = "luai_gcmetrics")]
use crate::records::gc_cycle_metrics::GCCycleMetrics;
use crate::{
  enums::lua_gc_op::LuaGcOp,
  functions::{lua_c_fullgc::lua_c_fullgc, lua_c_step::lua_c_step},
  macros::{condhardmemtests::condhardmemtests, gc_spause::GCSPAUSE},
  records::{global_state::global_State, lua_state::LuaState},
};

/// GC 参数以 KB 计：字节 = KB << KB_SHIFT（cpp 用字面量 10 / 1023）
const KB_SHIFT: u32 = 10;
const KB_MASK: usize = (1 << KB_SHIFT) - 1;
/// gc_threshold 的「GC 暂停」哨兵值
const GC_PAUSED_THRESHOLD: usize = usize::MAX;

/// # Safety
///
/// `l` must be a valid pointer to a live `LuaState`.
pub unsafe fn lua_gc(l: *mut LuaState, what: i32, data: i32) -> i32 {
  let mut res: i32 = 0;
  // Safety: 契约保证 `l` 为存活调用帧且 what/data 参数配对满足各分支约定（如 setpause 传指针或 null）
  unsafe {
    condhardmemtests!(lua_c_validate(l), 1);
    let g: *mut global_State = (*l).global;
    let Some(op) = LuaGcOp::from_repr(what) else {
      return -1;
    };
    match op {
      LuaGcOp::Stop => {
        (*g).gc_threshold = GC_PAUSED_THRESHOLD;
      }
      LuaGcOp::Restart => {
        (*g).gc_threshold = (*g).totalbytes;
      }
      LuaGcOp::Collect => {
        lua_c_fullgc(l);
      }
      LuaGcOp::Count => {
        res = ((*g).totalbytes >> KB_SHIFT) as i32;
      }
      LuaGcOp::Countb => {
        res = ((*g).totalbytes & KB_MASK) as i32;
      }
      LuaGcOp::Isrunning => {
        res = if (*g).gc_threshold != GC_PAUSED_THRESHOLD {
          1
        } else {
          0
        };
      }
      LuaGcOp::Step => {
        let amount: usize = (data as usize) << KB_SHIFT;
        let gcstate_i32: i32 = i32::from((*g).gcstate);

        let oldcredit: isize = if gcstate_i32 == GCSPAUSE {
          0
        } else {
          (*g).gc_threshold as isize - (*g).totalbytes as isize
        };

        // temporarily adjust the threshold so that we can perform GC work
        if amount <= (*g).totalbytes {
          (*g).gc_threshold = (*g).totalbytes - amount;
        } else {
          (*g).gc_threshold = 0;
        }

        #[cfg(feature = "luai_gcmetrics")]
        let startmarktime = (*g).gcmetrics.currcycle.marktime;
        #[cfg(feature = "luai_gcmetrics")]
        let startsweeptime = (*g).gcmetrics.currcycle.sweeptime;

        // track how much work the loop will actually perform
        let mut actualwork: usize = 0;

        loop {
          if (*g).gc_threshold > (*g).totalbytes {
            break;
          }
          let stepsize = lua_c_step(l, false);
          actualwork += stepsize;

          let gcstate_i32 = i32::from((*g).gcstate);
          if gcstate_i32 == GCSPAUSE {
            res = 1; // signal it
            break;
          }
        }

        #[cfg(feature = "luai_gcmetrics")]
        {
          // record explicit step statistics
          let cyclemetrics: &mut GCCycleMetrics = if i32::from((*g).gcstate) == GCSPAUSE {
            &mut (*g).gcmetrics.lastcycle
          } else {
            &mut (*g).gcmetrics.currcycle
          };

          let totalmarktime = cyclemetrics.marktime - startmarktime;
          let totalsweeptime = cyclemetrics.sweeptime - startsweeptime;

          if totalmarktime > 0.0 {
            cyclemetrics.markexplicitsteps += 1;

            if totalmarktime > cyclemetrics.markmaxexplicittime {
              cyclemetrics.markmaxexplicittime = totalmarktime;
            }
          }

          if totalsweeptime > 0.0 {
            cyclemetrics.sweepexplicitsteps += 1;

            if totalsweeptime > cyclemetrics.sweepmaxexplicittime {
              cyclemetrics.sweepmaxexplicittime = totalsweeptime;
            }
          }
        }

        // if cycle hasn't finished, advance threshold forward for the amount of extra work performed
        let gcstate_i32 = i32::from((*g).gcstate);
        if gcstate_i32 != GCSPAUSE {
          // if a new cycle was triggered by explicit step, old 'credit' of GC work is 0
          let newthreshold = (*g).totalbytes as isize + actualwork as isize + oldcredit;
          (*g).gc_threshold = if newthreshold < 0 {
            0
          } else {
            newthreshold as usize
          };
        }
      }
      LuaGcOp::Setgoal => {
        let res = (*g).gcgoal;
        (*g).gcgoal = data;
        return res;
      }
      LuaGcOp::Setstepmul => {
        let res = (*g).gcstepmul;
        (*g).gcstepmul = data;
        return res;
      }
      LuaGcOp::Setstepsize => {
        // GC values are expressed in Kbytes: #bytes/2^KB_SHIFT
        let res = (*g).gcstepsize >> KB_SHIFT;
        (*g).gcstepsize = data << KB_SHIFT;
        return res;
      }
      LuaGcOp::IsPaused => {
        // cpp lapi.cpp case LUA_GCISPAUSED：res = g->gcstate == GCSpause
        res = i32::from(i32::from((*g).gcstate) == GCSPAUSE);
      }
    }
  }
  res
}
