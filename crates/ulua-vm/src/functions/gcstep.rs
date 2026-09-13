use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    atomic::atomic, lua_clock::lua_clock, lua_m_getnextpage::luaM_getnextpage, markroot::markroot,
    propagatemark::propagatemark, shrinkbuffers::shrinkbuffers, sweepgcopage::sweepgcopage,
  },
  macros::{
    gc_satomic::{GCSATOMIC, GCSSWEEP},
    gc_spause::GCSPAUSE,
    gc_spropagate::{GCSPROPAGATE, GCSPROPAGATEAGAIN},
    gc_sweeppagestepcost::GC_SWEEPPAGESTEPCOST,
    isdead::isdead,
    makewhite::makewhite,
  },
  records::gc_object::GCObject,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn gcstep(l: *mut lua_State, limit: usize) -> usize {
  unsafe {
    let mut cost = 0usize;
    let g = (*l).global;

    match (*g).gcstate as i32 {
      GCSPAUSE => {
        markroot(l);
        LUAU_ASSERT!((*g).gcstate as i32 == GCSPROPAGATE);
      }
      GCSPROPAGATE => {
        while !(*g).gray.is_null() && cost < limit {
          cost += propagatemark(g);
        }

        if (*g).gray.is_null() {
          (*g).gray = (*g).grayagain;
          (*g).grayagain = null_mut();
          (*g).gcstate = GCSPROPAGATEAGAIN as u8;
        }
      }
      GCSPROPAGATEAGAIN => {
        while !(*g).gray.is_null() && cost < limit {
          cost += propagatemark(g);
        }

        if (*g).gray.is_null() {
          (*g).gcstate = GCSATOMIC as u8;
        }
      }
      GCSATOMIC => {
        (*g).gcstats.atomicstarttimestamp = lua_clock();
        (*g).gcstats.atomicstarttotalsizebytes = (*g).totalbytes;

        cost = atomic(l);

        LUAU_ASSERT!((*g).gcstate as i32 == GCSSWEEP);
      }
      GCSSWEEP => {
        while !(*g).sweepgcopage.is_null() && cost < limit {
          let next = luaM_getnextpage((*g).sweepgcopage);
          let steps = sweepgcopage(l, (*g).sweepgcopage);

          (*g).sweepgcopage = next;
          cost += steps as usize * GC_SWEEPPAGESTEPCOST as usize;
        }

        if (*g).sweepgcopage.is_null() {
          LUAU_ASSERT!(!isdead!(g, (*g).mainthread as *mut GCObject));
          makewhite!(g, (*g).mainthread as *mut GCObject);

          shrinkbuffers(l);

          (*g).gcstate = GCSPAUSE as u8;
        }
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }

    cost
  }
}
