use core::ptr::{addr_of_mut, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{gcstep::gcstep, markroot::markroot, shrinkbuffersfull::shrinkbuffersfull},
  macros::{
    gc_satomic::GCSSWEEP, gc_spause::GCSPAUSE, keepinvariant::keepinvariant, upisopen::upisopen,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_c_fullgc(l: *mut lua_State) {
  unsafe {
    let g = (*l).global;

    if keepinvariant(g) {
      (*g).sweepgcopage = (*g).allgcopages;
      (*g).gray = null_mut();
      (*g).grayagain = null_mut();
      (*g).weak = null_mut();
      (*g).gcstate = GCSSWEEP as u8;
    }

    LUAU_ASSERT!((*g).gcstate as i32 == GCSPAUSE || (*g).gcstate as i32 == GCSSWEEP);
    loop {
      if (*g).gcstate as i32 == GCSPAUSE {
        break;
      }
      LUAU_ASSERT!((*g).gcstate as i32 == GCSSWEEP);
      gcstep(l, usize::MAX);
    }

    let uvhead = addr_of_mut!((*g).uvhead);
    let mut uv = (*g).uvhead.u.open.next;
    while uv != uvhead {
      LUAU_ASSERT!(upisopen!(uv));
      (*uv).markedopen = 0;
      uv = (*uv).u.open.next;
    }

    markroot(l);
    loop {
      if (*g).gcstate as i32 == GCSPAUSE {
        break;
      }
      gcstep(l, usize::MAX);
    }

    shrinkbuffersfull(l);

    let heapgoalsizebytes = ((*g).totalbytes / 100) * (*g).gcgoal as usize;
    (*g).gc_threshold = (*g).totalbytes * (((*g).gcgoal * (*g).gcstepmul / 100 - 100) as usize)
      / (*g).gcstepmul as usize;

    if (*g).gc_threshold < (*g).totalbytes {
      (*g).gc_threshold = (*g).totalbytes;
    }

    (*g).gcstats.heapgoalsizebytes = heapgoalsizebytes;
  }
}

pub use lua_c_fullgc as luaC_fullgc;
