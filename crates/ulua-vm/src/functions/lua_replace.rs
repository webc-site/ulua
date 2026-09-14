use core::{ffi::c_int, ptr::eq};

use crate::{
  functions::{index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi},
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, hvalue::hvalue,
    lua_c_barrier::luaC_barrier, lua_environindex::LUA_ENVIRONINDEX,
    lua_globalsindex::LUA_GLOBALSINDEX, lua_o_nilobject::luaO_nilobject, setobj::setobj,
    ttistable::ttistable,
  },
  records::lua_state::lua_State,
  type_aliases::{closure::Closure, stk_id::StkId},
};

unsafe fn current_closure(l: *mut lua_State) -> *mut Closure {
  unsafe {
    let func = (*(*l).ci).func;
    core::ptr::addr_of_mut!((*(*func).value.gc).cl) as *mut Closure
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_replace(l: *mut lua_State, idx: c_int) {
  unsafe {
    api_checknelems!(l, 1);
    lua_c_threadbarrier_lapi(l);
    let o: StkId = index2addr(l, idx);
    api_check!(l, !eq(o, luaO_nilobject));
    if idx == LUA_ENVIRONINDEX {
      api_check!(l, (*l).ci != (*l).base_ci);
      let func: *mut Closure = current_closure(l);
      api_check!(l, ttistable!((*l).top.offset(-1)));
      (*func).env = hvalue!((*l).top.offset(-1));
      luaC_barrier!(l, func, (*l).top.offset(-1));
    } else if idx == LUA_GLOBALSINDEX {
      api_check!(l, ttistable!((*l).top.offset(-1)));
      (*l).gt = hvalue!((*l).top.offset(-1));
    } else {
      setobj!(l, o, (*l).top.offset(-1));
      if idx < LUA_GLOBALSINDEX {
        luaC_barrier!(l, current_closure(l), (*l).top.offset(-1));
      }
    }
    (*l).top = (*l).top.offset(-1);
  }
}
