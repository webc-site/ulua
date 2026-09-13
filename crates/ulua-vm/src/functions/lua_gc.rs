use core::ffi::c_int;

#[cfg(feature = "hard_mem_tests")]
use crate::functions::lua_c_validate::lua_c_validate;
#[cfg(feature = "luai_gcmetrics")]
use crate::records::gc_cycle_metrics::GCCycleMetrics;
use crate::{
  enums::lua_gc_op::LuaGcOp,
  functions::{lua_c_fullgc::lua_c_fullgc, lua_c_step::luaC_step},
  macros::{cast_int::cast_int, condhardmemtests::condhardmemtests, gc_spause::GCSPAUSE},
  records::global_state::global_State,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
pub unsafe fn lua_gc(l: *mut lua_State, what: c_int, data: c_int) -> c_int {
  let mut res: i32 = 0;
  unsafe {
    condhardmemtests!(lua_c_validate(l), 1);
    let g: *mut global_State = (*l).global;
    match what {
      x if x == LuaGcOp::Stop as i32 => {
        (*g).gc_threshold = usize::MAX;
      }
      x if x == LuaGcOp::Restart as i32 => {
        (*g).gc_threshold = (*g).totalbytes;
      }
      x if x == LuaGcOp::Collect as i32 => {
        lua_c_fullgc(l);
      }
      x if x == LuaGcOp::Count as i32 => {
        res = cast_int!((*g).totalbytes >> 10);
      }
      x if x == LuaGcOp::Countb as i32 => {
        res = cast_int!((*g).totalbytes & 1023);
      }
      x if x == LuaGcOp::Isrunning as i32 => {
        res = if (*g).gc_threshold != usize::MAX {
          1
        } else {
          0
        };
      }
      x if x == LuaGcOp::Step as i32 => {
        let amount: usize = (data as usize) << 10;
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
          let stepsize = luaC_step(l, false);
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
      x if x == LuaGcOp::Setgoal as i32 => {
        let res = (*g).gcgoal;
        (*g).gcgoal = data;
        return res;
      }
      x if x == LuaGcOp::Setstepmul as i32 => {
        let res = (*g).gcstepmul;
        (*g).gcstepmul = data;
        return res;
      }
      x if x == LuaGcOp::Setstepsize as i32 => {
        // GC values are expressed in Kbytes: #bytes/2^10
        let res = (*g).gcstepsize >> 10;
        (*g).gcstepsize = data << 10;
        return res;
      }
      _ => {
        return -1;
      }
    }
  }
  res
}
