use core::ffi::{c_int, c_void};

use crate::{
  functions::{
    lua_d_callint::lua_d_callint, lua_d_pcall::luaD_pcall, lua_isyieldable::lua_isyieldable,
  },
  macros::{
    api_check::api_check, c_call_yield::C_CALL_YIELD, clvalue::clvalue,
    expandstacklimit::expandstacklimit, iscfunction::iscfunction, isyielded::isyielded,
    lua_callinfo_handle::LUA_CALLINFO_HANDLE, savestack::savestack,
  },
  records::closure::CClosure,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

struct CallContext {
  func: StkId,
  nresults: c_int,
}

unsafe extern "C-unwind" fn pcallyieldable_run(l: *mut lua_State, ud: *mut c_void) {
  unsafe {
    let ctx = &*(ud as *const CallContext);
    let preparereentry = lua_isyieldable(l) != 0;
    lua_d_callint(l, ctx.func, ctx.nresults, preparereentry);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_pcallyieldable(
  l: *mut lua_State,
  nargs: c_int,
  nresults: c_int,
  errfunc: c_int,
) -> c_int {
  unsafe {
    api_check!(l, iscfunction!((*(*l).ci).func));
    let cl = clvalue!((*(*l).ci).func);
    let c = core::ptr::addr_of!((*cl).inner.c).cast::<CClosure>();
    api_check!(l, (*c).cont.is_some());
    api_check!(l, (nargs + 1) as isize <= (*l).top.offset_from((*l).base));
    api_check!(
      l,
      errfunc >= 0 && errfunc as isize <= (*l).top.offset_from((*l).base)
    );

    (*(*l).ci).errfunc = errfunc;
    (*(*l).ci).flags |= LUA_CALLINFO_HANDLE as u32;

    let mut ctx = CallContext {
      func: (*l).top.sub((nargs + 1) as usize),
      nresults,
    };

    let savedfunc = savestack!(l, ctx.func);
    let savederrfunc = if errfunc != 0 {
      savestack!(l, (*l).base.add((errfunc - 1) as usize))
    } else {
      0
    };

    let status = luaD_pcall(
      l,
      Some(pcallyieldable_run),
      core::ptr::addr_of_mut!(ctx).cast::<c_void>(),
      savedfunc as isize,
      savederrfunc as isize,
    );

    expandstacklimit!(l, (*l).top);

    if status == 0 && isyielded(l) {
      return C_CALL_YIELD;
    }

    (*(*l).ci).flags &= !(LUA_CALLINFO_HANDLE as u32);

    ((*c).cont.unwrap())(l, status)
  }
}

pub use lua_pcallyieldable as lua_l_pcallyieldable;
