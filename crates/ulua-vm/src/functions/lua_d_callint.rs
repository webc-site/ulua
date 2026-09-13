use core::ffi::c_int;

use crate::{
  functions::{lua_d_check_cstack::luaD_checkCstack, performcall::performcall},
  macros::{
    clvalue::clvalue, isyielded::isyielded, lua_c_check_gc::luaC_checkGC, lua_multret::LUA_MULTRET,
    luai_maxccalls::LUAI_MAXCCALLS, restoreci::restoreci, restorestack::restorestack,
    saveci::saveci, savestack::savestack,
  },
  records::closure::CClosure,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_d_callint(l: *mut lua_State, func: StkId, nresults: c_int, preparereentry: bool) {
  unsafe {
    (*l).n_ccalls = (*l).n_ccalls.wrapping_add(1);
    if (*l).n_ccalls as i32 >= LUAI_MAXCCALLS {
      luaD_checkCstack(l);
    }

    let mut fromyieldableccall = false;

    if (*l).ci != (*l).base_ci {
      let ccl = clvalue!((*(*l).ci).func);
      let cc = core::ptr::addr_of!((*ccl).inner.c).cast::<CClosure>();
      if (*ccl).is_c != 0 && (*cc).cont.is_some() {
        fromyieldableccall = true;
        (*l).base_ccalls = (*l).base_ccalls.wrapping_add(1);
      }
    }

    let funcoffset = savestack!(l, func);
    let cioffset = saveci!(l, (*l).ci);

    performcall(l, func, nresults, preparereentry);

    let yielded = isyielded(l);

    if fromyieldableccall {
      (*l).base_ccalls = (*l).base_ccalls.wrapping_sub(1);

      if yielded {
        let callerci = restoreci!(l, cioffset);
        (*callerci).top = restorestack!(l, funcoffset).add(if nresults != LUA_MULTRET {
          nresults as usize
        } else {
          0
        });
      }
    }

    if nresults != LUA_MULTRET && !yielded {
      (*l).top = restorestack!(l, funcoffset).add(nresults as usize);
    }

    (*l).n_ccalls = (*l).n_ccalls.wrapping_sub(1);
    luaC_checkGC!(l);
  }
}

pub use lua_d_callint as luaD_callint;
